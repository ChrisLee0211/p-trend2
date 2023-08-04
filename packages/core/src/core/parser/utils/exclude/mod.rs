use std::rc::Rc;
use globmatch::Builder;
pub struct ExcludeChecker<'a> {
    rules: Rc<Vec<Rc<Builder<'a>>>>,
  }
  
  impl<'a> ExcludeChecker<'a> {
    pub fn new<'b>(excludes: &'b Vec<String>) -> ExcludeChecker<'b> {
      let rule_checkers: Vec<Rc<Builder<'b>>> = excludes
        .iter()
        .map(|rule| return Rc::new(Builder::new(rule)))
        .collect();
      return ExcludeChecker {
        rules: Rc::new(rule_checkers),
      };
    }
  
    pub fn check(&self, path: &String) -> bool {
      let mut result = true;
      let rules = &self.rules;
      for rule in rules.iter() {
        let check_result = rule
          .build_glob()
          .expect("fail to load exclude rule in check method")
          .is_match(path);
        if check_result != result {
          result = check_result;
          break;
        }
      }
      return result;
    }
  }