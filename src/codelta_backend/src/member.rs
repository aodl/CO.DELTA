pub struct Member<'a> {
    pub principal: &'a str,
    pub account: &'a str,
}

pub struct Members<'a> {
    /// ### Rok
    pub aligatorr89: Member<'a>,
    /// ### Alex Lorimer
    pub lorimer: Member<'a>,
    /// ### Malith Hatananchchige
    pub malith_hatananchchige: Member<'a>,
}

pub const MEMBERS: Members = Members {
    aligatorr89: Member {
        account: "c5b791df89098320ed193f3e026f011c2999a1915764926a0a1a254a990b16ad",
        principal: "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae",
    },
    lorimer: Member {
        account: "f6a7fde8fed980f87e4c9ec6fe04820c9fd709a8a6e85deb6aea3c1c1d30c0df",
        principal: "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae",
    },
    malith_hatananchchige: Member {
        account: "a27050324650c2ec5d29a5a7003136c70608ddc166ead1c45656b3ab3c2bcf69",
        principal: "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae",
    },
};

impl Members<'_> {
    pub fn into_iter(&self) -> impl Iterator<Item = &Member> {
        [
            MEMBERS.aligatorr89,
            MEMBERS.lorimer,
            MEMBERS.malith_hatananchchige,
        ]
        .iter()
    }
}
