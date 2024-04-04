import React, { useState } from 'react';
import Element from './Element';

const ITEMS_PER_PAGE = 5;

const Sidebar = ({ sessions, onCardClick }) => {
    const [sidebarOpen, setSidebarOpen] = useState(true);
    const [currentPage, setCurrentPage] = useState(0);

    const toggleSidebar = () => {
        setSidebarOpen(!sidebarOpen);
    };

    const pageCount = Math.ceil(sessions.length / ITEMS_PER_PAGE);

    const sessionsToShow = sessions.slice(
        currentPage * ITEMS_PER_PAGE,
        (currentPage + 1) * ITEMS_PER_PAGE
    );

    const goToPage = (pageNumber) => {
        setCurrentPage(pageNumber);
    };

    return (
        <div className={`sidebar ${sidebarOpen ? 'open' : 'closed'}`}>
            <button onClick={toggleSidebar} className="toggle-button">
                {sidebarOpen ? '>' : '<'}
            </button>
            {sidebarOpen && (
                <>
                    <div className="card-container">
                        {sessionsToShow.map(session => (
                            <Element key={session.id} session={session} onClick={() => onCardClick(session.id)} />
                        ))}
                    </div>
                    <div className="pagination">
                        {Array.from({ length: pageCount }, (_, i) => (
                            <button
                                key={i}
                                onClick={() => goToPage(i)}
                                className={`page-item ${i === currentPage ? 'active' : ''}`}
                            >
                                {i + 1}
                            </button>
                        ))}
                    </div>
                </>
            )}
        </div>
    );
};

export default Sidebar;
