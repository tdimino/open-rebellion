
void __thiscall FUN_004c6830(void *this,int *param_1,uint *param_2,uint *param_3)

{
  if ((*param_2 & 0x20000000) != 0) {
    *(int *)((int)this + 0x60) = *(int *)((int)this + 0x60) + 1;
  }
  if ((*param_2 & 0x40000000) != 0) {
    *(int *)((int)this + 100) = *(int *)((int)this + 100) + 1;
  }
  if ((*param_2 & 4) == 0) {
    if ((*param_2 & 8) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40;
      *(int *)((int)this + 0x70) = *(int *)((int)this + 0x70) + 1;
      *(int *)((int)this + 0xbc) = *(int *)((int)this + 0xbc) + *param_1;
    }
    else {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x20;
      *(int *)((int)this + 0x6c) = *(int *)((int)this + 0x6c) + 1;
      *(int *)((int)this + 0xc0) = *(int *)((int)this + 0xc0) + *param_1;
    }
  }
  else {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10;
    *(int *)((int)this + 0x68) = *(int *)((int)this + 0x68) + 1;
    *(int *)((int)this + 0x78) = *(int *)((int)this + 0x78) + param_1[3];
    *(int *)((int)this + 0x7c) = *(int *)((int)this + 0x7c) + param_1[4];
    *(int *)((int)this + 0x80) = *(int *)((int)this + 0x80) + param_1[5];
    *(int *)((int)this + 0x84) = *(int *)((int)this + 0x84) + param_1[6];
    *(int *)((int)this + 0x88) = *(int *)((int)this + 0x88) + param_1[7];
    *(int *)((int)this + 0x8c) = *(int *)((int)this + 0x8c) + param_1[8];
    *(int *)((int)this + 0x90) = *(int *)((int)this + 0x90) + param_1[9];
    *(int *)((int)this + 0x94) = *(int *)((int)this + 0x94) + param_1[10];
    *(int *)((int)this + 0x98) = *(int *)((int)this + 0x98) + param_1[0xe];
    *(int *)((int)this + 0x9c) = *(int *)((int)this + 0x9c) + param_1[0xb];
    *(int *)((int)this + 0xa0) = *(int *)((int)this + 0xa0) + param_1[0xc];
    *(int *)((int)this + 0xa4) = *(int *)((int)this + 0xa4) + param_1[0xf];
    *(int *)((int)this + 0xa8) = *(int *)((int)this + 0xa8) + param_1[0x10];
    *(int *)((int)this + 0xb0) = *(int *)((int)this + 0xb0) + param_1[0x12];
    *(int *)((int)this + 0xac) = *(int *)((int)this + 0xac) + param_1[0x11];
    *(int *)((int)this + 0xb4) = *(int *)((int)this + 0xb4) + param_1[0x15];
    *(int *)((int)this + 0xb8) = *(int *)((int)this + 0xb8) + *param_1;
    if ((*param_2 & 0x20) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x80;
    }
    if ((*param_2 & 0x40) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x100;
    }
    if ((*param_2 & 0x80) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x1000;
    }
    if ((*param_2 & 0x100) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x2000;
    }
    if ((*param_2 & 0x200) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x4000;
    }
    if ((*param_2 & 0x400) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10000;
    }
    if ((*param_2 & 0x800) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x20000;
    }
    if ((*param_2 & 0x1000) == 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x40000;
    }
    if ((*param_2 & 0x10000000) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x1000000;
    }
    if ((*param_2 & 2) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x10000000;
    }
    if ((*param_2 & 1) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x20000000;
      *(int *)((int)this + 0xc4) = *(int *)((int)this + 0xc4) - param_1[0x14];
    }
    if ((*param_2 & 0x400) != 0) {
      *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x8000;
    }
  }
  if ((*param_3 & 8) != 0) {
    *(int *)((int)this + 0x74) = *(int *)((int)this + 0x74) + 1;
  }
  if ((*param_3 & 0x20000000) != 0) {
    *(uint *)((int)this + 0x28) = *(uint *)((int)this + 0x28) | 0x1000000;
  }
  if ((*param_2 & 0x80000000) != 0) {
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 0x2000000;
  }
  return;
}

