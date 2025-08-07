//! Solution for https://leetcode.com/problems/design-twitter
//! 355. Design Twitter

use std::collections::{HashMap, HashSet};

type UserId = i32;
type TweetID = i32;

#[derive(Default)]
struct Twitter {
    tweets: Vec<(UserId, TweetID)>,
    follow_list: HashMap<UserId, HashSet<UserId>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    const MAX_FEED: usize = 10;

    fn new() -> Self {
        Self::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut result = vec![];
        for (_tweet_user_id, tweet_id) in
            self.tweets
                .iter()
                .copied()
                .rev()
                .filter(|(tweet_user_id, _)| {
                    tweet_user_id == &user_id
                        || self
                            .follow_list
                            .get(&user_id)
                            .is_some_and(|followee| followee.contains(tweet_user_id))
                })
        {
            result.push(tweet_id);
            if result.len() >= Self::MAX_FEED {
                // Result at max length
                break;
            }
        }
        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_list
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_list
            .entry(follower_id)
            .or_default()
            .remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5); // User 1 posts a new tweet (id = 5).
        assert_eq!(twitter.get_news_feed(1), vec![5]); // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
        twitter.follow(1, 2); // User 1 follows user 2.
        twitter.post_tweet(2, 6); // User 2 posts a new tweet (id = 6).
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]); // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
        twitter.unfollow(1, 2); // User 1 unfollows user 2.
        assert_eq!(twitter.get_news_feed(1), vec![5]); // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
    }
}
