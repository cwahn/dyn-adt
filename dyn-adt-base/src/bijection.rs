use std::collections::HashMap;

pub struct Bijection<A, B>
where
    A: Eq + Clone + std::hash::Hash,
    B: Eq + Clone + std::hash::Hash,
{
    injection: HashMap<A, B>,
    surjection: HashMap<B, A>,
}

impl<A, B> Bijection<A, B>
where
    A: Eq + Clone + std::hash::Hash,
    B: Eq + Clone + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            injection: HashMap::new(),
            surjection: HashMap::new(),
        }
    }

    /// Define or extend the mapping: f(a) = b
    pub fn map(&mut self, a: &A, b: &B) -> Option<()> {
        if self.injection.contains_key(a) || self.surjection.contains_key(b) {
            return None;
        }
        self.injection.insert(a.clone(), b.clone());
        self.surjection.insert(b.clone(), a.clone());
        Some(())
    }

    /// Get image: given a in domain, find f(a)
    pub fn image(&self, a: &A) -> Option<&B> {
        self.injection.get(a)
    }

    /// Get preimage: given b in codomain, find f⁻¹(b)
    pub fn preimage(&self, b: &B) -> Option<&A> {
        self.surjection.get(b)
    }

    /// Remove a specific mapping f(a)=b
    pub fn unmap(&mut self, a: &A, b: &B) -> Option<()> {
        if self.injection.get(a) != Some(b) {
            return None;
        }
        self.injection.remove(a);
        self.surjection.remove(b);
        Some(())
    }

    /// Remove by domain element: remove f(a) and corresponding inverse mapping
    pub fn unmap_by_preimage(&mut self, a: &A) -> Option<()> {
        let b = self.injection.get(a)?;
        self.surjection.remove(b);
        self.injection.remove(a);
        Some(())
    }

    /// Remove by codomain element: remove f⁻¹(b) and corresponding forward mapping
    pub fn unmap_by_image(&mut self, b: &B) -> Option<()> {
        let a = self.surjection.get(b)?;
        self.injection.remove(a);
        self.surjection.remove(b);
        Some(())
    }

    /// Check if domain element is present
    pub fn domain_contains(&self, a: &A) -> bool {
        self.injection.contains_key(a)
    }

    /// Check if codomain element is present
    pub fn codomain_contains(&self, b: &B) -> bool {
        self.surjection.contains_key(b)
    }

    /// Clear all mappings
    pub fn clear(&mut self) {
        self.injection.clear();
        self.surjection.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bijection() {
        let mut bijection = Bijection::new();
        let a = 1;
        let b = 2;
        let c = 3;
        let d = 4;
        assert_eq!(bijection.map(&a, &b), Some(()));
        assert_eq!(bijection.map(&c, &d), Some(()));
        assert_eq!(bijection.map(&a, &c), None);
        assert_eq!(bijection.map(&c, &b), None);
        assert_eq!(bijection.image(&a), Some(&b));
        assert_eq!(bijection.image(&c), Some(&d));
        assert_eq!(bijection.preimage(&b), Some(&a));
        assert_eq!(bijection.preimage(&d), Some(&c));
        assert_eq!(bijection.unmap(&a, &b), Some(()));
        assert_eq!(bijection.image(&a), None);
        assert_eq!(bijection.preimage(&b), None);
        assert_eq!(bijection.unmap_by_preimage(&c), Some(()));
        assert_eq!(bijection.image(&c), None);
        assert_eq!(bijection.preimage(&d), None);
        assert_eq!(bijection.map(&a, &b), Some(()));
        assert_eq!(bijection.map(&c, &d), Some(()));
        assert_eq!(bijection.unmap_by_image(&b), Some(()));
        assert_eq!(bijection.image(&a), None);
        assert_eq!(bijection.preimage(&b), None);
        assert_eq!(bijection.clear(), ());
        assert_eq!(bijection.image(&a), None);
        assert_eq!(bijection.preimage(&b), None);
    }
}
