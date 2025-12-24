package org.example.proiectlab.repository;

import org.example.proiectlab.model.Symbol;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface SymbolRepository extends JpaRepository<Symbol, Long> {

    Optional<Symbol> findByTicker(String ticker);

    boolean existsByTicker(String ticker);
}

