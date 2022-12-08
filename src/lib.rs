use std::collections::HashMap;
use std::sync::Arc;

type PlayerID = u64;
type CardID = u64;
type TraitID = u64;
type FeatureID = u64;
type TraitValue = i64;
type CharacterIdx = usize;
type FeatureIdx = usize;

pub struct Card {
    pub name: String,
    pub description: String,
    pub effect: fn(
        &mut Encounter, player: CharacterIdx, target: CharacterIdx
    ),
}

pub struct Trait {
    pub name: String,
}

pub struct Feature {
    pub name: String,
    pub description: String,
    pub effect: fn(&mut Encounter, feature: FeatureIdx),
}

pub struct Deck {
    pub clist: Vec::<Option<CardID>>,
    pub deck: Vec::<CardID>,
    pub hand: Vec::<CardID>,
    pub discard: Vec::<CardID>,
}

impl Deck {
    pub fn draw_card(&mut self) {
        if self.deck.len() == 0 {
            std::mem::swap(
                &mut self.deck,
                &mut self.discard
            );
        }
        if let Some(&cid) = self.deck.last() {
            self.hand.push(cid);
            self.deck.pop();
        }
    }

    pub fn discard_card(&mut self, index: usize) {
        self.discard.push(self.hand[index]);
        self.hand.remove(index);
    }

    pub fn discard_hand(&mut self) {
        self.discard.append(&mut self.hand);
    }
}

pub struct Character {
    pub traits: HashMap::<TraitID,TraitValue>,
    pub deck: Deck,
}

pub struct Encounter {
    pub characters: Vec<Character>,
    pub features: Vec<FeatureID>,
    pub done: bool,
    pub card_list: Arc::<Vec::<Card>>,
    pub feature_list: Arc::<Vec::<Feature>>
}

impl Encounter {
    pub fn play_card(
        &mut self,
        pid: CharacterIdx,
        tid: CharacterIdx,
        i: usize
    ) -> bool {
        let max = self.characters.len();
        if pid < max && tid < max {
            let deck = &mut self.characters[pid].deck;
            if let Some(&cid) = deck.hand.get(i) {
                if let Some(Some(n))
                    = deck.clist.get(cid as usize)
                {
                    if let Some(cd) = self.card_list
                        .get((*n) as usize)
                    {
                        deck.discard_card(i);
                        (cd.effect)(
                            self,
                            pid,
                            tid
                        );
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn activate_feature(
        &mut self,
        fid: usize,
    ) -> bool {
        if let Some(n) = self.features.get(fid) {
            if let Some(feature) = self.feature_list
                .get((*n) as usize)
            {
                (feature.effect)(
                    self,
                    fid
                );
                return true;
            }
        }

        false
    }
}

pub struct Player {
    pub id: PlayerID,
    pub character: Character,
}

pub struct Game {
    pub players: HashMap::<PlayerID, CharacterIdx>,
    pub encounter: Encounter,
}

impl Game {
    pub fn new(
        players: Vec::<Player>,
        features: Vec::<FeatureID>,
        card_list: Arc::<Vec::<Card>>,
        feature_list: Arc::<Vec::<Feature>>
    ) -> Self {
        let mut player_map = HashMap::new();
        let mut characters = Vec::with_capacity(players.len());
        for player in players {
            player_map.insert(
                player.id, characters.len()
            );
            characters.push(player.character);
        }
        Game {
            players: player_map,
            encounter: Encounter {
                characters: characters,
                features: features,
                done: false,
                card_list: card_list,
                feature_list: feature_list,
            },
        }
    }

    pub fn get_character(
        &self, pid: PlayerID
    ) -> Option<&Character> {
        self.players.get(&pid).and_then(
            |&ch_id| -> Option<&Character> {
                self.encounter.characters.get(ch_id as usize)
            }
        )
    }

    pub fn get_mut_character(
        &mut self, pid: PlayerID
    ) -> Option<&mut Character> {
        self.players.get(&pid).and_then(
            |&ch_id| -> Option<&mut Character> {
                self.encounter.characters.get_mut(ch_id as usize)
            }
        )
    }
}
