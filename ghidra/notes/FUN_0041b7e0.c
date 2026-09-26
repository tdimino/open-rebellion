
void __thiscall FUN_0041b7e0(void *this,int param_1,uint *param_2,uint *param_3,uint *param_4)

{
  *(int *)((int)this + 0x1ac) = *(int *)((int)this + 0x1ac) + *(int *)(param_1 + 0xc);
  *(int *)((int)this + 0x1b0) = *(int *)((int)this + 0x1b0) + *(int *)(param_1 + 0x10);
  *(int *)((int)this + 0x1b4) = *(int *)((int)this + 0x1b4) + *(int *)(param_1 + 0x14);
  *(int *)((int)this + 0x1d0) = *(int *)((int)this + 0x1d0) + *(int *)(param_1 + 0x38);
  *(int *)((int)this + 0x1d4) = *(int *)((int)this + 0x1d4) + *(int *)(param_1 + 0x3c);
  *(int *)((int)this + 0x1d8) = *(int *)((int)this + 0x1d8) + *(int *)(param_1 + 0x8c);
  *(int *)((int)this + 0x1dc) = *(int *)((int)this + 0x1dc) + *(int *)(param_1 + 0x90);
  *(int *)((int)this + 0x1e0) = *(int *)((int)this + 0x1e0) + *(int *)(param_1 + 0x94);
  *(int *)((int)this + 0x1e8) = *(int *)((int)this + 0x1e8) + *(int *)(param_1 + 0x24);
  *(int *)((int)this + 0x1ec) = *(int *)((int)this + 0x1ec) + *(int *)(param_1 + 0x28);
  *(int *)((int)this + 0x1f0) = *(int *)((int)this + 0x1f0) + *(int *)(param_1 + 0xa0);
  *(int *)((int)this + 500) = *(int *)((int)this + 500) + *(int *)(param_1 + 0x98);
  *(int *)((int)this + 0x1f8) = *(int *)((int)this + 0x1f8) + *(int *)(param_1 + 0xc0);
  *(int *)((int)this + 0x1cc) = *(int *)((int)this + 0x1cc) + *(int *)(param_1 + 0x7c);
  *(int *)((int)this + 0x1fc) = *(int *)((int)this + 0x1fc) + *(int *)(param_1 + 0x2c);
  *(int *)((int)this + 0x200) = *(int *)((int)this + 0x200) + *(int *)(param_1 + 0x80);
  *(int *)((int)this + 0x204) = *(int *)((int)this + 0x204) + *(int *)(param_1 + 0x11c);
  *(int *)((int)this + 0x21c) = *(int *)((int)this + 0x21c) + *(int *)(param_1 + 0xf4);
  *(int *)((int)this + 0x220) = *(int *)((int)this + 0x220) + *(int *)(param_1 + 0x118);
  *(int *)((int)this + 0x224) = *(int *)((int)this + 0x224) + *(int *)(param_1 + 0x130);
  *(int *)((int)this + 0x214) = *(int *)((int)this + 0x214) + *(int *)(param_1 + 0xf8);
  *(int *)((int)this + 0x218) = *(int *)((int)this + 0x218) + *(int *)(param_1 + 0x108);
  if (*(int *)((int)this + 0x20c) < *(int *)(param_1 + 0xc4)) {
    *(int *)((int)this + 0x20c) = *(int *)(param_1 + 0xc4);
  }
  if (*(int *)((int)this + 0x208) < *(int *)(param_1 + 200)) {
    *(int *)((int)this + 0x208) = *(int *)(param_1 + 200);
  }
  if (*(int *)((int)this + 0x2fc) < *(int *)(param_1 + 0xd0)) {
    *(int *)((int)this + 0x2fc) = *(int *)(param_1 + 0xd0);
  }
  if ((*param_3 & 4) != 0) {
    *(int *)((int)this + 0x1b8) = *(int *)((int)this + 0x1b8) + 1;
    if ((*param_3 & 0x1000000) != 0) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x10;
    }
    if ((*param_3 & 0x2000000) != 0) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x20;
    }
    if ((*param_3 & 0x200) != 0) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x80;
    }
    if ((*param_4 & 0x8000000) != 0) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x100;
    }
  }
  if ((*param_3 & 8) != 0) {
    *(int *)((int)this + 0x1bc) = *(int *)((int)this + 0x1bc) + 1;
    if ((*(int *)(param_1 + 0x30) < *(int *)((int)this + 0x210)) ||
       (*(int *)((int)this + 0x210) == 0)) {
      *(int *)((int)this + 0x210) = *(int *)(param_1 + 0x30);
    }
  }
  if ((*param_3 & 0x10) != 0) {
    *(int *)((int)this + 0x1c0) = *(int *)((int)this + 0x1c0) + 1;
  }
  if ((*param_3 & 0x80000000) != 0) {
    *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 0x400;
  }
  if ((*param_2 & 0x40000000) == 0) {
    *(int *)((int)this + 0x1c4) = *(int *)((int)this + 0x1c4) + 1;
  }
  if ((*param_3 & 2) == 0) {
    *(int *)((int)this + 0x1c8) = *(int *)((int)this + 0x1c8) + 1;
  }
  return;
}

