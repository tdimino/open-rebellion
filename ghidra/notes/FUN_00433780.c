
void __thiscall FUN_00433780(void *this,int *param_1,uint *param_2)

{
  uint uVar1;
  int iVar2;
  
  *(int *)((int)this + 0x50) = *(int *)((int)this + 0x50) + 1;
  if ((*param_2 & 0x100) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x20000;
  }
  if ((*param_2 & 0x200) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x40000;
  }
  if ((*param_2 & 0x400000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x4000000;
  }
  if ((*param_2 & 0x20000000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x8000000;
  }
  if ((*param_2 & 0x800) == 0) {
    if ((*param_2 & 0x2000) == 0) {
      uVar1 = *(uint *)((int)this + 0x38) | 0x4000;
      goto LAB_004337e2;
    }
  }
  else {
    uVar1 = *(uint *)((int)this + 0x38) | 0x800;
LAB_004337e2:
    *(uint *)((int)this + 0x38) = uVar1;
  }
  if ((*param_2 & 0x10000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x200;
  }
  if ((*param_2 & 0x2000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x400;
  }
  if ((*param_2 & 0x8000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x100;
  }
  if ((*param_2 & 0x80) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x10;
  }
  if ((*param_2 & 0x400) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x20;
  }
  if ((*param_2 & 0x801000) == 0) {
    uVar1 = *(uint *)((int)this + 0x38) | 0x8000;
  }
  else {
    if ((*param_2 & 0x800000) != 0) {
      *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x2000;
    }
    if ((*param_2 & 0x1000) == 0) goto LAB_00433863;
    uVar1 = *(uint *)((int)this + 0x38) | 0x1000;
  }
  *(uint *)((int)this + 0x38) = uVar1;
LAB_00433863:
  if ((*param_2 & 0x801000) == 0) {
    if ((*param_2 & 1) != 0) {
      *(int *)((int)this + 0x98) = *(int *)((int)this + 0x98) + 1;
    }
    if ((*param_2 & 2) != 0) {
      *(int *)((int)this + 0x9c) = *(int *)((int)this + 0x9c) + 1;
    }
    if ((*param_2 & 4) != 0) {
      *(int *)((int)this + 0xa0) = *(int *)((int)this + 0xa0) + 1;
    }
  }
  if ((*param_2 & 0x80000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x1000000;
  }
  if ((*param_2 & 0x40000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x200000;
    *(int *)((int)this + 0xb0) = *(int *)((int)this + 0xb0) + param_1[0x12];
  }
  if ((*param_2 & 0x100000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x400000;
  }
  if ((*param_2 & 0x4000000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x2000000;
  }
  if ((*param_2 & 0x200000) != 0) {
    *(uint *)((int)this + 0x38) = *(uint *)((int)this + 0x38) | 0x800000;
  }
  if ((*param_2 & 0x8000000) == 0) {
    *(int *)((int)this + 0x94) = *(int *)((int)this + 0x94) + 1;
  }
  if ((*param_2 & 0x2000) == 0) {
    *(int *)((int)this + 0x58) = *(int *)((int)this + 0x58) + param_1[0xe];
    *(int *)((int)this + 0x5c) = *(int *)((int)this + 0x5c) + param_1[0xf];
    *(int *)((int)this + 0x60) = *(int *)((int)this + 0x60) + param_1[3];
    *(int *)((int)this + 0x68) = *(int *)((int)this + 0x68) + param_1[1];
    *(int *)((int)this + 0x6c) = *(int *)((int)this + 0x6c) + param_1[2];
    *(int *)((int)this + 0x54) = *(int *)((int)this + 0x54) + *param_1;
    *(int *)((int)this + 100) = *(int *)((int)this + 100) + param_1[0x10];
    *(int *)((int)this + 0x70) = *(int *)((int)this + 0x70) + param_1[5];
    *(int *)((int)this + 0x78) = *(int *)((int)this + 0x78) + param_1[7];
    *(int *)((int)this + 0x74) = *(int *)((int)this + 0x74) + param_1[6];
    *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + param_1[10];
    *(int *)((int)this + 0x7c) = *(int *)((int)this + 0x7c) + param_1[8];
    *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + param_1[9];
    *(int *)((int)this + 0xa4) = *(int *)((int)this + 0xa4) + param_1[0x11];
    *(int *)((int)this + 0xb4) = *(int *)((int)this + 0xb4) + param_1[0x13];
    *(int *)((int)this + 0xb8) = *(int *)((int)this + 0xb8) + param_1[0x14];
  }
  *(int *)((int)this + 0x88) = *(int *)((int)this + 0x88) + param_1[0xb];
  *(int *)((int)this + 0x8c) = *(int *)((int)this + 0x8c) + param_1[0xc];
  *(int *)((int)this + 0x90) = *(int *)((int)this + 0x90) + param_1[0xd];
  if ((*param_2 & 4) != 0) {
    *(int *)((int)this + 0xbc) = *(int *)((int)this + 0xbc) + param_1[10];
  }
  if ((*param_2 & 0x2800) == 0) {
    iVar2 = *(int *)((int)this + 0xa8) + param_1[10];
    *(int *)((int)this + 0xa8) = iVar2;
    *(int *)((int)this + 0xa8) = iVar2 - param_1[8];
    iVar2 = *(int *)((int)this + 0xac) + param_1[7];
    *(int *)((int)this + 0xac) = iVar2;
    *(int *)((int)this + 0xac) = iVar2 - param_1[5];
  }
  return;
}

