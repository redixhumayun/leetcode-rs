pub struct BrowserHistory {
    current: Vec<String>,
    forward: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        let mut current = Vec::new();
        current.push(homepage);
        BrowserHistory {
            current,
            forward: Vec::new(),
        }
    }

    pub fn visit(&mut self, url: String) {
        self.forward.clear();
        self.current.push(url);
    }

    pub fn back(&mut self, steps: i32) -> String {
        let mut index = 0;
        while let Some(url) = self.current.pop() {
            if self.current.len() == 0 {
                self.current.push(url.clone());
                return url;
            }
            self.forward.push(url);
            index += 1;
            if index == steps {
                break;
            }
        }
        if let Some(url) = self.current.last() {
            return url.to_string();
        }
        return "".to_string();
    }

    pub fn forward(&mut self, steps: i32) -> String {
        let mut index = 0;
        while let Some(url) = self.forward.pop() {
            self.current.push(url);
            index += 1;
            if index == steps {
                break;
            }
        }
        if let Some(url) = self.current.last() {
            return url.to_string();
        }
        return "".to_string();
    }
}
