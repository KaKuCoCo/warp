use settings_page::MatchData;

use super::*;

// ── SettingsSection classification ──────────────────────────────────────────

#[test]
fn ai_subpages_are_identified() {
    assert!(SettingsSection::WarpAgent.is_ai_subpage());
    assert!(SettingsSection::AgentProfiles.is_ai_subpage());
    assert!(SettingsSection::AgentMCPServers.is_ai_subpage());
    assert!(SettingsSection::Knowledge.is_ai_subpage());
    assert!(SettingsSection::ThirdPartyCLIAgents.is_ai_subpage());

    assert!(!SettingsSection::AI.is_ai_subpage());
    assert!(!SettingsSection::Account.is_ai_subpage());
    assert!(!SettingsSection::CodeIndexing.is_ai_subpage());
}

#[test]
fn code_subpages_are_identified() {
    assert!(SettingsSection::CodeIndexing.is_code_subpage());
    assert!(SettingsSection::EditorAndCodeReview.is_code_subpage());

    assert!(!SettingsSection::Code.is_code_subpage());
    assert!(!SettingsSection::WarpAgent.is_code_subpage());
}

#[test]
fn cloud_platform_subpages_are_identified() {
    assert!(SettingsSection::CloudEnvironments.is_cloud_platform_subpage());
    assert!(SettingsSection::OzCloudAPIKeys.is_cloud_platform_subpage());

    assert!(!SettingsSection::Account.is_cloud_platform_subpage());
    assert!(!SettingsSection::WarpAgent.is_cloud_platform_subpage());
}

#[test]
fn is_subpage_covers_all_umbrella_types() {
    // All subpages under any umbrella should return true.
    for section in SettingsSection::ai_subpages() {
        assert!(section.is_subpage(), "{section:?} should be a subpage");
    }
    assert!(SettingsSection::CodeIndexing.is_subpage());
    assert!(SettingsSection::EditorAndCodeReview.is_subpage());
    assert!(SettingsSection::CloudEnvironments.is_subpage());
    assert!(SettingsSection::OzCloudAPIKeys.is_subpage());

    // Top-level pages should not be subpages.
    assert!(!SettingsSection::Account.is_subpage());
    assert!(!SettingsSection::AI.is_subpage());
    assert!(!SettingsSection::Code.is_subpage());
    assert!(!SettingsSection::Privacy.is_subpage());
}

// ── parent_page_section mapping ─────────────────────────────────────────────

#[test]
fn ai_subpages_map_to_ai_backing_page() {
    assert_eq!(
        SettingsSection::WarpAgent.parent_page_section(),
        SettingsSection::AI
    );
    assert_eq!(
        SettingsSection::AgentProfiles.parent_page_section(),
        SettingsSection::AI
    );
    assert_eq!(
        SettingsSection::Knowledge.parent_page_section(),
        SettingsSection::AI
    );
    assert_eq!(
        SettingsSection::ThirdPartyCLIAgents.parent_page_section(),
        SettingsSection::AI
    );
}

#[test]
fn agent_mcp_servers_maps_to_mcp_servers_page() {
    // AgentMCPServers renders the standalone MCPServers page, not the AI page.
    assert_eq!(
        SettingsSection::AgentMCPServers.parent_page_section(),
        SettingsSection::MCPServers
    );
}

#[test]
fn code_subpages_map_to_code_backing_page() {
    assert_eq!(
        SettingsSection::CodeIndexing.parent_page_section(),
        SettingsSection::Code
    );
    assert_eq!(
        SettingsSection::EditorAndCodeReview.parent_page_section(),
        SettingsSection::Code
    );
}

#[test]
fn cloud_platform_subpages_map_to_their_backing_pages() {
    assert_eq!(
        SettingsSection::CloudEnvironments.parent_page_section(),
        SettingsSection::CloudEnvironments
    );
    assert_eq!(
        SettingsSection::OzCloudAPIKeys.parent_page_section(),
        SettingsSection::OzCloudAPIKeys
    );
}

#[test]
fn non_subpage_sections_map_to_themselves() {
    assert_eq!(
        SettingsSection::Account.parent_page_section(),
        SettingsSection::Account
    );
    assert_eq!(
        SettingsSection::AI.parent_page_section(),
        SettingsSection::AI
    );
    assert_eq!(
        SettingsSection::Privacy.parent_page_section(),
        SettingsSection::Privacy
    );
}

// ── ai_subpages list ────────────────────────────────────────────────────────

#[test]
fn ai_subpages_list_contains_only_local_visible_ai_subpages() {
    let subpages = SettingsSection::ai_subpages();
    assert_eq!(subpages, &[SettingsSection::ThirdPartyCLIAgents]);
    assert!(!subpages.contains(&SettingsSection::WarpAgent));
    assert!(!subpages.contains(&SettingsSection::AgentProfiles));
    assert!(!subpages.contains(&SettingsSection::AgentMCPServers));
    assert!(!subpages.contains(&SettingsSection::Knowledge));
}

#[test]
fn ai_subpages_list_does_not_contain_non_subpages() {
    let subpages = SettingsSection::ai_subpages();
    assert!(!subpages.contains(&SettingsSection::AI));
    assert!(!subpages.contains(&SettingsSection::Account));
    assert!(!subpages.contains(&SettingsSection::Code));
}

#[test]
fn local_warp_cloud_ui_hides_official_cloud_and_agent_sections() {
    assert!(is_local_warp_cloud_ui_disabled());

    for section in [
        SettingsSection::Account,
        SettingsSection::BillingAndUsage,
        SettingsSection::Teams,
        SettingsSection::Referrals,
        SettingsSection::SharedBlocks,
        SettingsSection::WarpDrive,
        SettingsSection::WarpAgent,
        SettingsSection::AgentProfiles,
        SettingsSection::AgentMCPServers,
        SettingsSection::Knowledge,
        SettingsSection::CloudEnvironments,
        SettingsSection::OzCloudAPIKeys,
    ] {
        assert!(
            section.is_hidden_by_local_warp_cloud_ui(),
            "{section:?} should be hidden by the local cloud/agent UI patch"
        );
    }

    for section in [
        SettingsSection::ThirdPartyCLIAgents,
        SettingsSection::MCPServers,
        SettingsSection::Appearance,
        SettingsSection::Features,
        SettingsSection::Keybindings,
        SettingsSection::Privacy,
        SettingsSection::About,
    ] {
        assert!(
            !section.is_hidden_by_local_warp_cloud_ui(),
            "{section:?} should remain visible in local mode"
        );
    }
}

#[test]
fn local_warp_cloud_ui_redirects_hidden_entrypoints_to_safe_pages() {
    assert_eq!(
        SettingsSection::AI.local_warp_cloud_ui_fallback(),
        SettingsSection::ThirdPartyCLIAgents
    );
    assert_eq!(
        SettingsSection::WarpAgent.local_warp_cloud_ui_fallback(),
        SettingsSection::ThirdPartyCLIAgents
    );
    assert_eq!(
        SettingsSection::AgentMCPServers.local_warp_cloud_ui_fallback(),
        SettingsSection::MCPServers
    );
    assert_eq!(
        SettingsSection::BillingAndUsage.local_warp_cloud_ui_fallback(),
        SettingsSection::Appearance
    );
    assert_eq!(
        SettingsSection::Scripting.local_warp_cloud_ui_fallback(),
        SettingsSection::Scripting
    );
}

// ── MatchData behavior ──────────────────────────────────────────────────────

#[test]
fn match_data_uncounted_true_is_truthy() {
    assert!(MatchData::Uncounted(true).is_truthy());
}

#[test]
fn match_data_uncounted_false_is_not_truthy() {
    assert!(!MatchData::Uncounted(false).is_truthy());
}

#[test]
fn match_data_countable_nonzero_is_truthy() {
    assert!(MatchData::Countable(3).is_truthy());
    assert!(MatchData::Countable(1).is_truthy());
}

#[test]
fn match_data_countable_zero_is_not_truthy() {
    assert!(!MatchData::Countable(0).is_truthy());
}

// ── Display / FromStr round-trip ────────────────────────────────────────────

#[test]
fn subpage_display_names_are_correct() {
    assert_eq!(SettingsSection::WarpAgent.to_string(), "Warp Agent");
    assert_eq!(SettingsSection::AgentProfiles.to_string(), "Profiles");
    assert_eq!(SettingsSection::AgentMCPServers.to_string(), "MCP servers");
    assert_eq!(SettingsSection::Knowledge.to_string(), "Knowledge");
    assert_eq!(
        SettingsSection::ThirdPartyCLIAgents.to_string(),
        "Third party CLI agents"
    );
    assert_eq!(
        SettingsSection::CodeIndexing.to_string(),
        "Indexing and projects"
    );
    assert_eq!(
        SettingsSection::EditorAndCodeReview.to_string(),
        "Editor and Code Review"
    );
    assert_eq!(
        SettingsSection::CloudEnvironments.to_string(),
        "Environments"
    );
    assert_eq!(
        SettingsSection::OzCloudAPIKeys.to_string(),
        "Oz Cloud API Keys"
    );
}

#[test]
fn subpage_from_str_parses_display_names() {
    // Both the legacy "Oz" name and the new "Warp Agent" display name must
    // resolve to SettingsSection::WarpAgent so existing deep links, persisted
    // telemetry strings, and external callers continue to work after the
    // user-facing rename (see specs/GH1063/product.md, Behavior #8).
    assert_eq!(
        SettingsSection::from_str("Oz"),
        Ok(SettingsSection::WarpAgent)
    );
    assert_eq!(
        SettingsSection::from_str("Warp Agent"),
        Ok(SettingsSection::WarpAgent)
    );
    assert_eq!(
        SettingsSection::from_str("Profiles"),
        Ok(SettingsSection::AgentProfiles)
    );
    assert_eq!(
        SettingsSection::from_str("Knowledge"),
        Ok(SettingsSection::Knowledge)
    );
    assert_eq!(
        SettingsSection::from_str("Indexing and projects"),
        Ok(SettingsSection::CodeIndexing)
    );
    assert_eq!(
        SettingsSection::from_str("Editor and Code Review"),
        Ok(SettingsSection::EditorAndCodeReview)
    );
    assert_eq!(
        SettingsSection::from_str("Oz Cloud API Keys"),
        Ok(SettingsSection::OzCloudAPIKeys)
    );
}

// ── Subpage search filter simulation ────────────────────────────────────────
// These tests simulate the per-subpage search filtering logic used in
// handle_search_editor_event: each subpage should only be visible if its
// own widgets' search terms match, not if a sibling subpage's terms match.

/// Helper: given a map of subpage→MatchData, returns which subpages are visible.
fn visible_subpages(
    subpage_filter: &HashMap<SettingsSection, MatchData>,
    subpages: &[SettingsSection],
) -> Vec<SettingsSection> {
    subpages
        .iter()
        .filter(|s| {
            subpage_filter
                .get(s)
                .map(|md| md.is_truthy())
                .unwrap_or(false)
        })
        .copied()
        .collect()
}

#[test]
fn search_knowledge_does_not_show_hidden_official_agent_subpage() {
    // Local patch mode removes Knowledge from the visible Agents subpage list.
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(1));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(0),
    );

    let visible = visible_subpages(&filter, SettingsSection::ai_subpages());

    assert!(visible.is_empty());
}

#[test]
fn search_agent_shows_only_cli_agents_subpage() {
    // Local patch mode hides official AgentProfiles and leaves only CLI agents visible.
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(2));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(1),
    );

    let visible = visible_subpages(&filter, SettingsSection::ai_subpages());

    assert_eq!(visible, vec![SettingsSection::ThirdPartyCLIAgents]);
}

#[test]
fn empty_search_shows_no_subpages_in_filter() {
    // When search is cleared, subpage_filter is empty — all subpages fall back
    // to their backing page visibility (Uncounted(true) by default).
    let filter: HashMap<SettingsSection, MatchData> = HashMap::new();

    let visible = visible_subpages(&filter, SettingsSection::ai_subpages());

    // No entries in filter means no subpage-specific filtering; all return false
    // from the filter map. The actual rendering code falls back to the backing
    // page's pages_filter which defaults to Uncounted(true).
    assert!(visible.is_empty());
}

#[test]
fn search_with_no_matches_hides_all_subpages() {
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(0),
    );

    let visible = visible_subpages(&filter, SettingsSection::ai_subpages());

    assert!(visible.is_empty());
}

/// Helper: check if an umbrella should be visible given a subpage filter.
fn umbrella_visible(
    subpage_filter: &HashMap<SettingsSection, MatchData>,
    umbrella_subpages: &[SettingsSection],
) -> bool {
    umbrella_subpages.iter().any(|s| {
        subpage_filter
            .get(s)
            .map(|md| md.is_truthy())
            .unwrap_or(false)
    })
}

#[test]
fn umbrella_hidden_when_no_subpages_match() {
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(0),
    );

    assert!(!umbrella_visible(&filter, SettingsSection::ai_subpages()));
}

// ── cycle_pages search filter ────────────────────────────────────────────────
// These tests validate the logic added to cycle_pages() to ensure arrow key
// navigation respects the active search filter.

/// Mirrors the filter predicate used in cycle_pages() when search is active.
fn section_passes_nav_filter(
    section: SettingsSection,
    subpage_filter: &HashMap<SettingsSection, MatchData>,
    pages_filter: &[(SettingsSection, MatchData)],
) -> bool {
    if let Some(md) = subpage_filter.get(&section) {
        md.is_truthy()
    } else {
        let backing = section.parent_page_section();
        pages_filter
            .iter()
            .any(|(s, md)| *s == backing && md.is_truthy())
    }
}

#[test]
fn nav_filter_includes_matching_subpage_and_excludes_others() {
    let mut subpage_filter = HashMap::new();
    subpage_filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    subpage_filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    subpage_filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    subpage_filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(1),
    );

    // No page-level filter entries needed since all AI subpages have subpage_filter entries.
    let pages_filter: Vec<(SettingsSection, MatchData)> = vec![];

    assert!(!section_passes_nav_filter(
        SettingsSection::WarpAgent,
        &subpage_filter,
        &pages_filter
    ));
    assert!(!section_passes_nav_filter(
        SettingsSection::AgentProfiles,
        &subpage_filter,
        &pages_filter
    ));
    assert!(!section_passes_nav_filter(
        SettingsSection::Knowledge,
        &subpage_filter,
        &pages_filter
    ));
    assert!(section_passes_nav_filter(
        SettingsSection::ThirdPartyCLIAgents,
        &subpage_filter,
        &pages_filter
    ));
}

#[test]
fn nav_filter_falls_back_to_pages_filter_for_top_level_pages() {
    // Top-level pages (Account, Appearance, etc.) have no subpage_filter entry.
    // They fall back to pages_filter using parent_page_section() == themselves.
    let subpage_filter: HashMap<SettingsSection, MatchData> = HashMap::new();
    let pages_filter = vec![
        (SettingsSection::Account, MatchData::Uncounted(true)),
        (SettingsSection::Appearance, MatchData::Countable(0)),
        (SettingsSection::Features, MatchData::Uncounted(true)),
    ];

    assert!(section_passes_nav_filter(
        SettingsSection::Account,
        &subpage_filter,
        &pages_filter
    ));
    assert!(!section_passes_nav_filter(
        SettingsSection::Appearance,
        &subpage_filter,
        &pages_filter
    ));
    assert!(section_passes_nav_filter(
        SettingsSection::Features,
        &subpage_filter,
        &pages_filter
    ));
}

#[test]
fn umbrella_visible_when_any_subpage_matches() {
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(1),
    );

    assert!(umbrella_visible(&filter, SettingsSection::ai_subpages()));
}

// ── Search auto-select simulation ───────────────────────────────────────────
// These tests simulate the auto-select logic in handle_search_editor_event:
// when the current subpage is filtered out by search, the view should jump
// to the first visible subpage or page.

/// Simulates the "is current still visible" check from the search handler.
/// Returns true if `current` is still visible given the subpage_filter and
/// a list of (backing_section, is_truthy) pairs for pages_filter.
fn is_current_visible(
    current: SettingsSection,
    subpage_filter: &HashMap<SettingsSection, MatchData>,
    pages_visible: &[(SettingsSection, bool)],
) -> bool {
    if let Some(md) = subpage_filter.get(&current) {
        return md.is_truthy();
    }
    let backing = current.parent_page_section();
    pages_visible
        .iter()
        .any(|(section, visible)| *section == backing && *visible)
}

/// Simulates finding the first visible section from the nav_items order.
fn first_visible_section(
    nav_order: &[SettingsSection],
    subpage_filter: &HashMap<SettingsSection, MatchData>,
    pages_visible: &[(SettingsSection, bool)],
) -> Option<SettingsSection> {
    nav_order.iter().copied().find(|section| {
        if let Some(md) = subpage_filter.get(section) {
            md.is_truthy()
        } else {
            let backing = section.parent_page_section();
            pages_visible
                .iter()
                .any(|(s, visible)| *s == backing && *visible)
        }
    })
}

#[test]
fn auto_select_jumps_away_from_filtered_out_subpage() {
    // User is on hidden Knowledge; local nav can only select ThirdPartyCLIAgents.
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(2));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(1),
    );

    let current = SettingsSection::Knowledge;
    assert!(
        !is_current_visible(current, &filter, &[]),
        "Knowledge should not be visible when it has 0 matches"
    );

    // The local Agents nav order only includes ThirdPartyCLIAgents.
    let nav_order = SettingsSection::ai_subpages();
    let first = first_visible_section(nav_order, &filter, &[]);
    assert_eq!(
        first,
        Some(SettingsSection::ThirdPartyCLIAgents),
        "Should auto-select CLI agents as the only visible local Agents subpage"
    );
}

#[test]
fn auto_select_stays_on_current_when_it_matches() {
    // User is on ThirdPartyCLIAgents, searches "agent" which matches the CLI agent page.
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(1),
    );

    let current = SettingsSection::ThirdPartyCLIAgents;
    assert!(
        is_current_visible(current, &filter, &[]),
        "ThirdPartyCLIAgents should remain visible when it has matches"
    );
}

#[test]
fn auto_select_falls_back_to_top_level_page_when_no_subpages_match() {
    // All AI subpages filtered out, but Appearance (top-level) is still visible.
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));
    filter.insert(SettingsSection::Knowledge, MatchData::Countable(0));
    filter.insert(
        SettingsSection::ThirdPartyCLIAgents,
        MatchData::Countable(0),
    );

    let pages_visible = vec![
        (SettingsSection::Appearance, true),
        (SettingsSection::AI, false),
    ];

    // Local nav order includes the Agents subpage before retained top-level pages.
    let nav_order = vec![
        SettingsSection::ThirdPartyCLIAgents,
        SettingsSection::Appearance,
    ];

    let first = first_visible_section(&nav_order, &filter, &pages_visible);
    assert_eq!(
        first,
        Some(SettingsSection::Appearance),
        "Should fall back to Appearance when no visible Agents subpage matches"
    );
}

#[test]
fn auto_select_handles_mcp_servers_top_level_page() {
    let filter = HashMap::new();

    let pages_visible = vec![
        (SettingsSection::MCPServers, true),
        (SettingsSection::AI, false),
    ];

    let current = SettingsSection::MCPServers;
    assert!(
        is_current_visible(current, &filter, &pages_visible),
        "MCPServers should remain visible as a top-level local settings page"
    );
}

#[test]
fn auto_select_with_no_matches_anywhere() {
    let mut filter = HashMap::new();
    filter.insert(SettingsSection::WarpAgent, MatchData::Countable(0));
    filter.insert(SettingsSection::AgentProfiles, MatchData::Countable(0));

    let pages_visible = vec![
        (SettingsSection::Appearance, false),
        (SettingsSection::AI, false),
    ];

    let nav_order = vec![
        SettingsSection::ThirdPartyCLIAgents,
        SettingsSection::Appearance,
    ];

    let first = first_visible_section(&nav_order, &filter, &pages_visible);
    assert_eq!(
        first, None,
        "No section should be selected when nothing matches"
    );
}

// ── Backward compatibility ──────────────────────────────────────────────────

#[test]
fn legacy_ai_section_keeps_ai_backing_page() {
    // SettingsSection::AI remains the backing page. Local navigation redirects
    // AI entrypoints to ThirdPartyCLIAgents in SettingsView.
    // Here we just verify the parent_page_section is still AI (for page lookup).
    assert_eq!(
        SettingsSection::AI.parent_page_section(),
        SettingsSection::AI
    );
    // And that AI is NOT itself a subpage.
    assert!(!SettingsSection::AI.is_subpage());
}

// ── Collapsed umbrella nav-stop behavior ────────────────────────────────────
// Verify that arrow-key navigation lands on a collapsed umbrella as a single
// stop (and activates it by jumping to the first subpage, which auto-expands
// the umbrella) instead of silently skipping over it.

use nav::{SettingsNavItem, SettingsUmbrella};

/// Builds the nav-items layout used by `SettingsView::new`, matching the real
/// sidebar ordering so tests exercise realistic nav orders.
fn realistic_nav_items() -> Vec<SettingsNavItem> {
    vec![
        SettingsNavItem::Umbrella(SettingsUmbrella::new(
            "Agents",
            SettingsSection::ai_subpages().to_vec(),
        )),
        SettingsNavItem::Page(SettingsSection::MCPServers),
        SettingsNavItem::Umbrella(SettingsUmbrella::new(
            "Code",
            SettingsSection::code_subpages().to_vec(),
        )),
        SettingsNavItem::Page(SettingsSection::Appearance),
        SettingsNavItem::Page(SettingsSection::Features),
        SettingsNavItem::Page(SettingsSection::Keybindings),
        SettingsNavItem::Page(SettingsSection::Warpify),
        SettingsNavItem::Page(SettingsSection::Privacy),
        SettingsNavItem::Page(SettingsSection::About),
    ]
}

/// Mutably flips an umbrella's `expanded` flag at `nav_index`.
fn set_expanded(nav_items: &mut [SettingsNavItem], nav_index: usize, expanded: bool) {
    if let Some(SettingsNavItem::Umbrella(u)) = nav_items.get_mut(nav_index) {
        u.expanded = expanded;
    } else {
        panic!("nav_items[{nav_index}] is not an Umbrella");
    }
}

#[test]
fn collapsed_umbrella_is_a_single_nav_stop() {
    let nav_items = realistic_nav_items();
    // All umbrellas default to collapsed.
    let stops = build_nav_stops(&nav_items, |_| true);

    // Expect: <Agents umbrella>, MCPServers, <Code umbrella>, Appearance,
    // Features, Keybindings, Warpify, Privacy, About.
    assert_eq!(stops.len(), 9);
    assert!(matches!(
        stops[0],
        NavStop::CollapsedUmbrella {
            nav_index: 0,
            first_subpage: SettingsSection::ThirdPartyCLIAgents,
            last_subpage: SettingsSection::ThirdPartyCLIAgents,
        }
    ));
    assert!(matches!(
        stops[1],
        NavStop::Section(SettingsSection::MCPServers)
    ));
    assert!(matches!(
        stops[2],
        NavStop::CollapsedUmbrella {
            nav_index: 2,
            first_subpage: SettingsSection::CodeIndexing,
            last_subpage: SettingsSection::EditorAndCodeReview,
        }
    ));
    assert!(matches!(
        stops[3],
        NavStop::Section(SettingsSection::Appearance)
    ));
    assert!(matches!(stops[8], NavStop::Section(SettingsSection::About)));
}

#[test]
fn expanded_umbrella_produces_section_stop_per_subpage() {
    let mut nav_items = realistic_nav_items();
    set_expanded(&mut nav_items, 0, true);

    let stops = build_nav_stops(&nav_items, |_| true);

    let sections: Vec<_> = stops
        .iter()
        .map(|s| match s {
            NavStop::Section(section) => format!("{section:?}"),
            NavStop::CollapsedUmbrella { nav_index, .. } => format!("Umbrella@{nav_index}"),
        })
        .collect();
    assert_eq!(
        sections,
        vec![
            "ThirdPartyCLIAgents",
            "MCPServers",
            "Umbrella@2",
            "Appearance",
            "Features",
            "Keybindings",
            "Warpify",
            "Privacy",
            "About",
        ]
    );
}

#[test]
fn collapsed_umbrella_with_filtered_subpages_uses_first_visible_subpage() {
    let nav_items = realistic_nav_items();

    let stops = build_nav_stops(&nav_items, |section| {
        section != SettingsSection::CodeIndexing
    });

    let code_stop = stops
        .iter()
        .find(|s| matches!(s, NavStop::CollapsedUmbrella { nav_index: 2, .. }))
        .expect("Code umbrella should still be a collapsed stop");

    match code_stop {
        NavStop::CollapsedUmbrella {
            first_subpage,
            last_subpage,
            ..
        } => {
            assert_eq!(
                *first_subpage,
                SettingsSection::EditorAndCodeReview,
                "CodeIndexing is hidden by the filter, so the first visible subpage is EditorAndCodeReview"
            );
            assert_eq!(
                *last_subpage,
                SettingsSection::EditorAndCodeReview,
                "Only one Code subpage remains visible"
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn umbrella_with_no_visible_subpages_is_skipped_entirely() {
    let nav_items = realistic_nav_items();

    let stops = build_nav_stops(&nav_items, |section| !section.is_ai_subpage());

    // The Agents umbrella's subpages are all AI subpages, so the entire
    // umbrella should be absent from the nav order.
    assert!(
        stops
            .iter()
            .all(|s| !matches!(s, NavStop::CollapsedUmbrella { nav_index: 0, .. })),
        "Agents umbrella should not appear when none of its subpages are visible"
    );
    // The still-visible Code umbrella remains as a stop.
    assert!(stops
        .iter()
        .any(|s| matches!(s, NavStop::CollapsedUmbrella { nav_index: 2, .. })));
}

#[test]
fn filtered_out_top_level_page_is_skipped() {
    let nav_items = realistic_nav_items();

    let stops = build_nav_stops(&nav_items, |section| section != SettingsSection::Appearance);

    assert!(
        !stops
            .iter()
            .any(|s| matches!(s, NavStop::Section(SettingsSection::Appearance))),
        "Appearance should be filtered out entirely"
    );
    // But other pages remain.
    assert!(stops
        .iter()
        .any(|s| matches!(s, NavStop::Section(SettingsSection::MCPServers))));
}

// ── current_stop_index ──────────────────────────────────────────────────────

#[test]
fn current_stop_index_matches_section_stop() {
    let nav_items = realistic_nav_items();
    let stops = build_nav_stops(&nav_items, |_| true);

    let idx = current_stop_index(&stops, &nav_items, SettingsSection::MCPServers);
    assert_eq!(idx, Some(1));
}

#[test]
fn current_stop_index_maps_subpage_to_collapsed_umbrella() {
    // Edge case: the user manually collapsed the Agents umbrella while still
    // on one of its subpages. The collapsed umbrella should match as the
    // current stop so arrow-key cycling continues from the umbrella's position.
    let nav_items = realistic_nav_items();
    let stops = build_nav_stops(&nav_items, |_| true);

    let idx = current_stop_index(&stops, &nav_items, SettingsSection::ThirdPartyCLIAgents);
    assert_eq!(
        idx,
        Some(0),
        "ThirdPartyCLIAgents is under the collapsed Agents umbrella at nav_index 0"
    );
}

#[test]
fn current_stop_index_returns_none_when_section_is_not_present() {
    let nav_items = realistic_nav_items();
    // Filter out all AI subpages (and therefore the Agents umbrella) entirely.
    let stops = build_nav_stops(&nav_items, |section| !section.is_ai_subpage());

    // WarpAgent isn't directly in stops, and no remaining collapsed umbrella
    // contains it, so current_stop_index should return None.
    assert_eq!(
        current_stop_index(&stops, &nav_items, SettingsSection::WarpAgent),
        None
    );
}

// ── next_stop_index wrapping ────────────────────────────────────────────────

#[test]
fn next_stop_index_wraps_at_ends() {
    assert_eq!(next_stop_index(0, 3, CycleDirection::Up), 2);
    assert_eq!(next_stop_index(2, 3, CycleDirection::Down), 0);
    assert_eq!(next_stop_index(1, 3, CycleDirection::Up), 0);
    assert_eq!(next_stop_index(1, 3, CycleDirection::Down), 2);
}

#[test]
fn next_stop_index_handles_single_stop() {
    assert_eq!(next_stop_index(0, 1, CycleDirection::Up), 0);
    assert_eq!(next_stop_index(0, 1, CycleDirection::Down), 0);
}

// ── End-to-end cycling (no search) ──────────────────────────────────────────
// These tests simulate the sequence of nav-stop activations that would result
// from repeatedly pressing Down/Up, ensuring a collapsed umbrella is never
// skipped over.

/// Computes the section that would become active after applying the direction
/// once, starting from `current`. Mirrors the final target-resolution step in
/// `cycle_pages`.
fn simulate_cycle(
    nav_items: &[SettingsNavItem],
    stops: &[NavStop],
    current: SettingsSection,
    direction: CycleDirection,
) -> SettingsSection {
    let active = current_stop_index(stops, nav_items, current)
        .expect("current should exist in stops in these tests");
    let next = next_stop_index(active, stops.len(), direction);
    match stops[next] {
        NavStop::Section(section) => section,
        NavStop::CollapsedUmbrella {
            first_subpage,
            last_subpage,
            ..
        } => match direction {
            CycleDirection::Up => last_subpage,
            CycleDirection::Down => first_subpage,
        },
    }
}

#[test]
fn arrow_down_from_about_wraps_to_collapsed_agents_first_subpage() {
    let nav_items = realistic_nav_items();
    let stops = build_nav_stops(&nav_items, |_| true);

    let next = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::About,
        CycleDirection::Down,
    );
    assert_eq!(next, SettingsSection::ThirdPartyCLIAgents);
}

#[test]
fn arrow_up_from_mcp_servers_with_collapsed_agents_lands_on_last_subpage() {
    let nav_items = realistic_nav_items();
    let stops = build_nav_stops(&nav_items, |_| true);

    let next = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::MCPServers,
        CycleDirection::Up,
    );
    assert_eq!(next, SettingsSection::ThirdPartyCLIAgents);
}

#[test]
fn arrow_up_into_collapsed_umbrella_respects_search_filter_for_last_subpage() {
    let nav_items = realistic_nav_items();
    let is_visible = |section: SettingsSection| section != SettingsSection::CodeIndexing;
    let stops = build_nav_stops(&nav_items, is_visible);

    let next = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::Appearance,
        CycleDirection::Up,
    );
    assert_eq!(next, SettingsSection::EditorAndCodeReview);
}

#[test]
fn arrow_down_from_expanded_last_subpage_leaves_umbrella() {
    let mut nav_items = realistic_nav_items();
    set_expanded(&mut nav_items, 0, true); // expand Agents
    let stops = build_nav_stops(&nav_items, |_| true);

    // ThirdPartyCLIAgents is the last Agents subpage; Down should move to
    // MCPServers (the next top-level page in the nav order).
    let next = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::ThirdPartyCLIAgents,
        CycleDirection::Down,
    );
    assert_eq!(next, SettingsSection::MCPServers);
}

#[test]
fn arrow_down_from_mcp_servers_enters_collapsed_code_umbrella() {
    let nav_items = realistic_nav_items();
    let stops = build_nav_stops(&nav_items, |_| true);

    let next_after_mcp = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::MCPServers,
        CycleDirection::Down,
    );
    assert_eq!(next_after_mcp, SettingsSection::CodeIndexing);

    let next_after_code = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::CodeIndexing,
        CycleDirection::Down,
    );
    assert_eq!(next_after_code, SettingsSection::Appearance);
}

#[test]
fn arrow_down_collapsed_umbrella_respects_search_filter() {
    let nav_items = realistic_nav_items();
    let is_visible = |section: SettingsSection| section != SettingsSection::CodeIndexing;
    let stops = build_nav_stops(&nav_items, is_visible);

    let next = simulate_cycle(
        &nav_items,
        &stops,
        SettingsSection::MCPServers,
        CycleDirection::Down,
    );
    assert_eq!(next, SettingsSection::EditorAndCodeReview);
}
