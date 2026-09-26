
void __thiscall FUN_0041ba80(void *this,undefined4 param_1,int param_2,uint *param_3)

{
  if ((*param_3 & 1) != 0) {
    if ((*param_3 & 4) != 0) {
      *(int *)((int)this + 0x268) = *(int *)((int)this + 0x268) + *(int *)(param_2 + 0x48);
      *(int *)((int)this + 0x26c) = *(int *)((int)this + 0x26c) + *(int *)(param_2 + 0x4c);
      *(int *)((int)this + 0x270) = *(int *)((int)this + 0x270) + *(int *)(param_2 + 0x50);
      *(int *)((int)this + 0x274) = *(int *)((int)this + 0x274) + *(int *)(param_2 + 100);
      *(int *)((int)this + 0x278) = *(int *)((int)this + 0x278) + *(int *)(param_2 + 0x68);
      *(int *)((int)this + 0x27c) = *(int *)((int)this + 0x27c) + *(int *)(param_2 + 0x28);
      *(int *)((int)this + 0x280) = *(int *)((int)this + 0x280) + *(int *)(param_2 + 0x18);
    }
    *(int *)((int)this + 0x25c) = *(int *)((int)this + 0x25c) + *(int *)(param_2 + 0x2c);
    *(int *)((int)this + 0x260) = *(int *)((int)this + 0x260) + *(int *)(param_2 + 0x6c);
    *(int *)((int)this + 600) = *(int *)((int)this + 600) + *(int *)(param_2 + 100);
    *(int *)((int)this + 0x24c) = *(int *)((int)this + 0x24c) + *(int *)(param_2 + 0x38);
    *(int *)((int)this + 0x250) = *(int *)((int)this + 0x250) + *(int *)(param_2 + 0x3c);
    *(int *)((int)this + 0x254) = *(int *)((int)this + 0x254) + *(int *)(param_2 + 0x40);
    if ((*param_3 & 0x10) != 0) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 4;
    }
    if ((*param_3 & 0x20) != 0) {
      *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 8;
    }
    *(int *)((int)this + 0x264) = *(int *)((int)this + 0x264) + *(int *)(param_2 + 0x44);
    return;
  }
  *(int *)((int)this + 0x284) = *(int *)((int)this + 0x284) + 1;
  *(int *)((int)this + 0x288) = *(int *)((int)this + 0x288) + *(int *)(param_2 + 100);
  *(int *)((int)this + 0x28c) = *(int *)((int)this + 0x28c) + *(int *)(param_2 + 0x28);
  *(int *)((int)this + 0x290) = *(int *)((int)this + 0x290) + *(int *)(param_2 + 0x68);
  if (*(int *)((int)this + 0x294) < *(int *)(param_2 + 100)) {
    *(int *)((int)this + 0x294) = *(int *)(param_2 + 100);
    *(undefined4 *)((int)this + 0x298) = *(undefined4 *)(param_2 + 0x68);
    *(undefined4 *)((int)this + 0x29c) = *(undefined4 *)(param_2 + 0x28);
  }
  return;
}

