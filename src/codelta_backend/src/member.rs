pub struct Member<'a> {
    pub principal: &'a str,
    pub account: &'a str,
}

pub struct Members<'a> {
    /// ### Rok
    pub aligatorr89: Member<'a>,
    /// ### Donna Powell
    pub donna: Member<'a>,
    /// ### Gabriel
    pub gabriel: Member<'a>,
    /// ### Gautier Wojda
    pub gautier: Member<'a>,
    /// ### Ethan Celletti
    pub gekctek: Member<'a>,
    /// ### Alex Lorimer
    pub lorimer: Member<'a>,
    /// ### Malith Hatananchchige
    pub malith_hatananchchige: Member<'a>,
    /// ### rem.codes
    pub remco: Member<'a>,
    /// ### zane
    pub zane: Member<'a>,
}

pub const MEMBERS: Members = Members {
    aligatorr89: Member {
        account: "c5b791df89098320ed193f3e026f011c2999a1915764926a0a1a254a990b16ad",
        principal: "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae",
    },
    donna: Member {
        account: "638138b167d49001a7da9dfa756d013a18bffaf87c1a60df13bc103ea86cdc2c",
        principal: "s653w-iizlk-et4s2-5wdju-2zflm-zm54k-bac3k-h7gdt-qltws-lji73-tqe",
    },
    gabriel: Member {
        account: "18292147c160aca36a08c5504eef0a75505fffad4f5351220403f01d8c641840",
        principal: "bn6wo-xpofx-5va6n-knhsi-d26er-6oxej-a5m3i-i5yh7-h3il7-s65zr-lae",
    },
    gautier: Member {
        account: "794e3a581d78757b60258d77905eaa8af04bd9d3b33fddce1c5ef2b0562d851e",
        principal: "yvi2m-qclpo-iof7c-xbzh5-4g2hb-i36yy-yx7i2-iczo2-oei56-ldao3-rae",
    },
    gekctek: Member {
        account: "9c22b78fe64f0f8d2c957c2fe071ce14c340374dbca8624396272ce578fa3f50",
        principal: "pib55-fsiwt-ftxf3-a6e7q-ed7dm-qfbgq-tdld3-jrotf-7y5bf-xsyju-uqe",
    },
    lorimer: Member {
        account: "f6a7fde8fed980f87e4c9ec6fe04820c9fd709a8a6e85deb6aea3c1c1d30c0df",
        principal: "zkkkd-i34qc-367ln-e2u7o-ezznu-dkfqh-gtfvz-cviph-6qa4v-evtfs-wqe",
    },
    malith_hatananchchige: Member {
        account: "a27050324650c2ec5d29a5a7003136c70608ddc166ead1c45656b3ab3c2bcf69",
        principal: "hfbtd-e2vzk-rvwfx-c55l3-tsuue-oizir-hl4bg-tajby-skikk-iefse-fqe",
    },
    remco: Member {
        account: "7272012b9b9460887f469ba9b337a86eb9aacdaf764c42b2ce5afe4b95ead3c3",
        principal: "mrzkb-iqvzd-crtjw-g2fai-hapm4-hlchs-exq4o-in7jm-szk73-o7bjn-cqe",
    },
    zane: Member {
        account: "ae6c23cdb9fa6dd3d6fbd8585f8ffea3d72de69fb4bae901c4dbd6e393d79dc2",
        principal: "3sbdf-73gy7-fe4ta-ycey4-r2sgo-usien-c7d7u-yznr5-fficl-azvb7-fqe",
    },
};

impl Members<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &Member> {
        [
            MEMBERS.aligatorr89,
            MEMBERS.donna,
            MEMBERS.gabriel,
            MEMBERS.gautier,
            MEMBERS.gekctek,
            MEMBERS.lorimer,
            MEMBERS.malith_hatananchchige,
            MEMBERS.remco,
            MEMBERS.zane,
        ]
        .iter()
    }
}
