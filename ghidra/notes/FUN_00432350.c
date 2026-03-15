
void __thiscall FUN_00432350(void *this,int *param_1,uint *param_2,uint *param_3,uint *param_4)

{
  uint uVar1;
  
  if ((*param_3 & 1) == 0) {
    if ((*param_3 & 4) == 0) {
      *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x10;
      *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + 1;
      *(int *)((int)this + 400) = *(int *)((int)this + 400) + param_1[0x27];
      *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + param_1[0x28];
      *(int *)((int)this + 0x198) = *(int *)((int)this + 0x198) + param_1[0x2b];
      *(int *)((int)this + 0x11c) = *(int *)((int)this + 0x11c) + param_1[4];
      *(int *)((int)this + 0x120) = *(int *)((int)this + 0x120) + param_1[5];
      *(int *)((int)this + 0x124) = *(int *)((int)this + 0x124) + param_1[6];
      if ((*param_2 & 0x80000000) != 0) {
        *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40000000;
      }
      if ((*param_3 & 0x20) == 0) goto LAB_00432b38;
      uVar1 = *(uint *)((int)this + 0x2c) | 0x40;
    }
    else {
      *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 8;
      *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + 1;
      *(int *)((int)this + 0x114) = *(int *)((int)this + 0x114) + param_1[2];
      *(int *)((int)this + 0x118) = *(int *)((int)this + 0x118) + param_1[3];
      *(int *)((int)this + 0x128) = *(int *)((int)this + 0x128) + param_1[7];
      *(int *)((int)this + 300) = *(int *)((int)this + 300) + param_1[10];
      *(int *)((int)this + 0x110) = *(int *)((int)this + 0x110) + (param_1[1] - param_1[0xf]);
      *(int *)((int)this + 400) = *(int *)((int)this + 400) + param_1[0x27];
      *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + param_1[0x28];
      *(int *)((int)this + 0x198) = *(int *)((int)this + 0x198) + param_1[0x2b];
      *(int *)((int)this + 0xa0) = *(int *)((int)this + 0xa0) + param_1[0x34];
      *(int *)((int)this + 0x130) = *(int *)((int)this + 0x130) + param_1[0x21];
      *(int *)((int)this + 0x11c) = *(int *)((int)this + 0x11c) + param_1[4];
      *(int *)((int)this + 0x120) = *(int *)((int)this + 0x120) + param_1[5];
      *(int *)((int)this + 0x124) = *(int *)((int)this + 0x124) + param_1[6];
      if (*(int *)((int)this + 0x134) < param_1[0x34]) {
        *(int *)((int)this + 0x134) = param_1[0x34];
      }
      if (*(int *)((int)this + 0x138) < param_1[0x17]) {
        *(int *)((int)this + 0x138) = param_1[0x17];
      }
      if (*(int *)((int)this + 0x13c) < param_1[0x1b]) {
        *(int *)((int)this + 0x13c) = param_1[0x1b];
      }
      if (*(int *)((int)this + 0x140) < param_1[0x1d]) {
        *(int *)((int)this + 0x140) = param_1[0x1d];
      }
      if ((*param_2 & 0x80000000) != 0) {
        *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x80000000;
      }
      if ((*param_3 & 0x800) != 0) {
        *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x100;
        if (param_1[8] < *(int *)((int)this + 0x144)) {
          *(int *)((int)this + 0x144) = param_1[8];
        }
      }
      if ((*param_3 & 0x100) == 0) {
        *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x400;
      }
      if ((*param_3 & 0x2000) == 0) {
        *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x400000;
      }
      if ((*param_4 & 0x2f70000) == 0) goto LAB_00432b38;
      uVar1 = *(uint *)((int)this + 0x2c) | 0x80000;
    }
    *(uint *)((int)this + 0x2c) = uVar1;
    goto LAB_00432b38;
  }
  *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 4;
  *(int *)((int)this + 0x7c) = *(int *)((int)this + 0x7c) + 1;
  *(int *)((int)this + 0x8c) = *(int *)((int)this + 0x8c) + *param_1;
  *(int *)((int)this + 0x90) = *(int *)((int)this + 0x90) + param_1[1];
  *(int *)((int)this + 0xa8) = *(int *)((int)this + 0xa8) + param_1[2];
  *(int *)((int)this + 0xac) = *(int *)((int)this + 0xac) + param_1[3];
  *(int *)((int)this + 200) = *(int *)((int)this + 200) + param_1[7];
  *(int *)((int)this + 0xcc) = *(int *)((int)this + 0xcc) + param_1[10];
  *(int *)((int)this + 0xd0) = *(int *)((int)this + 0xd0) + param_1[0xb];
  *(int *)((int)this + 0xd4) = *(int *)((int)this + 0xd4) + param_1[0x17];
  *(int *)((int)this + 0x98) = *(int *)((int)this + 0x98) + param_1[0xe];
  *(int *)((int)this + 0x94) = *(int *)((int)this + 0x94) + param_1[0xf];
  *(int *)((int)this + 0x9c) = *(int *)((int)this + 0x9c) + param_1[0x16];
  *(int *)((int)this + 0xf0) = *(int *)((int)this + 0xf0) + param_1[0x10];
  *(int *)((int)this + 0xfc) = *(int *)((int)this + 0xfc) + param_1[0x1e];
  *(int *)((int)this + 0x100) = *(int *)((int)this + 0x100) + param_1[0x1f];
  *(int *)((int)this + 0x104) = *(int *)((int)this + 0x104) + param_1[0x20];
  *(int *)((int)this + 0x108) = *(int *)((int)this + 0x108) + param_1[0x21];
  *(int *)((int)this + 0xdc) = *(int *)((int)this + 0xdc) + param_1[0x1a];
  *(int *)((int)this + 400) = *(int *)((int)this + 400) + param_1[0x27];
  *(int *)((int)this + 0x194) = *(int *)((int)this + 0x194) + param_1[0x28];
  *(int *)((int)this + 0x198) = *(int *)((int)this + 0x198) + param_1[0x2b];
  if ((*param_3 & 0x40000) != 0) {
    *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x800000;
    if ((*param_3 & 0x10000) == 0) {
      if ((*param_3 & 0x20000) == 0) goto LAB_0043251a;
      uVar1 = *(uint *)((int)this + 0x2c) | 0x2000000;
    }
    else {
      uVar1 = *(uint *)((int)this + 0x2c) | 0x1000000;
    }
    *(uint *)((int)this + 0x2c) = uVar1;
  }
LAB_0043251a:
  if ((*param_2 & 2) == 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 8;
  }
  if ((((*param_2 & 3) == 0) && ((*param_3 & 0x8000000) == 0)) && (0 < param_1[0x33])) {
    if ((*param_2 & 0x200000) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x20;
    }
    if ((*param_2 & 0x400000) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40;
    }
    if ((*param_2 & 0x3800000) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x80;
    }
  }
  if (((*param_2 & 0x80) != 0) && ((*param_2 & 3) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x100;
  }
  if (((*param_2 & 0x100) != 0) && ((*param_2 & 3) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x200;
  }
  if (((*param_2 & 0x20) != 0) && ((*param_2 & 3) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400;
  }
  if (((*param_2 & 0x40) != 0) && ((*param_2 & 3) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x800;
  }
  uVar1 = *param_2;
  if ((((uVar1 & 0x800000) != 0) && ((uVar1 & 0x800) != 0)) && ((uVar1 & 3) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x1000;
  }
  if (((*param_2 & 0x800) != 0) && ((*param_2 & 0x3800000) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x2000;
  }
  if ((*param_2 & 0x3e00000) == 0) {
LAB_00432660:
    if ((*param_2 & 0x3800000) == 0) goto LAB_00432668;
  }
  else {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x20000000;
    if ((*param_2 & 0x200000) != 0) {
      if ((*param_2 & 2) == 0) {
        *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x20000;
      }
      *(int *)((int)this + 0xb0) = *(int *)((int)this + 0xb0) + param_1[4];
    }
    if ((*param_2 & 0x400000) != 0) {
      *(int *)((int)this + 0xb8) = *(int *)((int)this + 0xb8) + param_1[5];
    }
    if ((*param_2 & 0x3800000) != 0) {
      *(int *)((int)this + 0xc0) = *(int *)((int)this + 0xc0) + param_1[6];
      goto LAB_00432660;
    }
LAB_00432668:
    *(int *)((int)this + 0xc4) = *(int *)((int)this + 0xc4) + param_1[6];
  }
  if ((*param_2 & 0x400000) == 0) {
    *(int *)((int)this + 0xbc) = *(int *)((int)this + 0xbc) + param_1[5];
  }
  if ((*param_2 & 0x200000) == 0) {
    *(int *)((int)this + 0xb4) = *(int *)((int)this + 0xb4) + param_1[4];
  }
  if (((*param_3 & 0x10000000) != 0) &&
     (*(int *)((int)this + 0x188) = *(int *)((int)this + 0x188) + 1, (*param_3 & 0x20000000) == 0))
  {
    *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x10000000;
  }
  if (param_1[0x25] < 1) {
    if (param_1[0x25] < 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x8000;
      *(int *)((int)this + 0xe0) = *(int *)((int)this + 0xe0) + 1;
      if ((*param_2 & 0x3e00000) != 0) {
        *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x4000;
      }
    }
  }
  else {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10000;
    *(int *)((int)this + 0xe4) = *(int *)((int)this + 0xe4) + 1;
  }
  if ((((*param_2 & 0x80000000) == 0) && (0 < param_1[0xb])) && (0 < param_1[7])) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x20000;
  }
  if ((*param_2 & 0x10000) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x80000;
    *(int *)((int)this + 0xec) = *(int *)((int)this + 0xec) + param_1[0x13];
    if ((*param_2 & 0x10000000) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x100000;
    }
    if ((*param_2 & 0x20000000) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x200000;
    }
    if ((*param_3 & 0x4000) != 0) {
      *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x800;
    }
    if ((*param_3 & 0x8000) != 0) {
      *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x1000;
    }
  }
  if ((*param_2 & 0x2000) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40000;
    *(int *)((int)this + 0xe8) = *(int *)((int)this + 0xe8) + param_1[0x15];
    if ((*param_2 & 0x3e00000) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x800000;
    }
  }
  if ((*param_2 & 0x20000) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x400000;
  }
  if (((0 < param_1[0x35]) && ((*param_3 & 0x40000) == 0)) && ((*param_2 & 2) == 0)) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x2000000;
    *(int *)((int)this + 0xf8) = *(int *)((int)this + 0xf8) + param_1[0x35];
  }
  if (param_1[0x26] < 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x1000000;
    *(int *)((int)this + 0xf4) = *(int *)((int)this + 0xf4) + 1;
    if ((*param_2 & 0x3e00000) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x4000000;
    }
    if ((*param_2 & 0x100000) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10000000;
    }
  }
  if ((*param_2 & 0x80000) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x8000000;
  }
  if ((*param_2 & 1) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10;
  }
  if ((*param_2 & 0x80000000) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40000000;
    if ((*param_3 & 0x40000000) != 0) {
      *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x200;
    }
    if ((*param_2 & 2) == 0) {
      *(int *)((int)this + 0x1a0) = *(int *)((int)this + 0x1a0) + 1;
    }
  }
  if ((*param_4 & 0x10000000) != 0) {
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x8000000;
  }
LAB_00432b38:
  *(int *)((int)this + 0x19c) = *(int *)((int)this + 0x19c) + param_1[0x32];
  if ((*param_4 & 0x8000000) != 0) {
    *(int *)((int)this + 0x70) = *(int *)((int)this + 0x70) + 1;
  }
  if ((*param_3 & 8) != 0) {
    *(int *)((int)this + 0x74) = *(int *)((int)this + 0x74) + 1;
  }
  if ((*param_3 & 0x20) != 0) {
    *(int *)((int)this + 0x78) = *(int *)((int)this + 0x78) + 1;
  }
  if ((*param_3 & 0x10) != 0) {
    *(int *)((int)this + 0x88) = *(int *)((int)this + 0x88) + 1;
  }
  if ((*param_3 & 0x40) != 0) {
    *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x20;
  }
  if ((*param_3 & 0x40000000) != 0) {
    *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x40000000;
  }
  if ((*param_3 & 0x80000000) != 0) {
    *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x80000000;
  }
  if ((*param_3 & 0x200) != 0) {
    *(uint *)((int)this + 0x2c) = *(uint *)((int)this + 0x2c) | 0x40000;
  }
  if ((*param_4 & 1) != 0) {
    *(int *)((int)this + 0x150) = *(int *)((int)this + 0x150) + 1;
  }
  if ((*param_4 & 2) != 0) {
    *(int *)((int)this + 0x160) = *(int *)((int)this + 0x160) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 8;
  }
  if ((*param_4 & 4) != 0) {
    *(int *)((int)this + 0x154) = *(int *)((int)this + 0x154) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 1;
  }
  if ((*param_4 & 8) != 0) {
    *(int *)((int)this + 0x158) = *(int *)((int)this + 0x158) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 2;
  }
  if ((*param_4 & 0x10) != 0) {
    *(int *)((int)this + 0x15c) = *(int *)((int)this + 0x15c) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 4;
  }
  if ((*param_4 & 0x20) != 0) {
    *(int *)((int)this + 0x164) = *(int *)((int)this + 0x164) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x10;
  }
  if ((*param_4 & 0x40) != 0) {
    *(int *)((int)this + 0x168) = *(int *)((int)this + 0x168) + 1;
    uVar1 = *(uint *)((int)this + 0x28);
    *(uint *)((int)this + 0x28) = uVar1 | 0x20;
    if ((*param_4 & 0x20000000) == 0) {
      *(uint *)((int)this + 0x28) = uVar1 | 0x4000020;
    }
  }
  if ((*param_4 & 0x80) != 0) {
    *(int *)((int)this + 0x16c) = *(int *)((int)this + 0x16c) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x40;
  }
  if ((*param_4 & 0xc00) != 0) {
    *(int *)((int)this + 0x170) = *(int *)((int)this + 0x170) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x80;
  }
  if ((*param_4 & 0x3000) != 0) {
    *(int *)((int)this + 0x174) = *(int *)((int)this + 0x174) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x100;
  }
  if ((*param_4 & 0x100) != 0) {
    *(int *)((int)this + 0x178) = *(int *)((int)this + 0x178) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x200;
  }
  if ((*param_4 & 0x200) != 0) {
    *(int *)((int)this + 0x17c) = *(int *)((int)this + 0x17c) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x400;
  }
  if ((*param_4 & 0x10000) != 0) {
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x800;
  }
  if ((*param_4 & 0x20000) != 0) {
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x1000;
  }
  if ((*param_4 & 0x800000) != 0) {
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x2000;
  }
  if ((*param_4 & 0xc000) != 0) {
    *(int *)((int)this + 0x180) = *(int *)((int)this + 0x180) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x4000;
  }
  if ((*param_4 & 0x1000000) != 0) {
    *(int *)((int)this + 0x184) = *(int *)((int)this + 0x184) + 1;
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x8000;
  }
  return;
}

