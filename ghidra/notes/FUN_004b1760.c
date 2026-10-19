
void __thiscall FUN_004b1760(void *this,uint *param_1)

{
  uint uVar1;
  void *pvVar2;
  
  *param_1 = *param_1 | 0x4000000;
  pvVar2 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x44),
                        (uint *)(*(int *)((int)this + 0x10) + 0x34));
  if (pvVar2 != (void *)0x0) {
    uVar1 = *param_1;
    *param_1 = uVar1 | 0x8000000;
    if (((*(uint *)((int)pvVar2 + 0x2c) & 2) == 0) || ((*(uint *)((int)pvVar2 + 0x2c) & 8) != 0)) {
      *param_1 = uVar1 | 0x8020000;
    }
  }
  return;
}

