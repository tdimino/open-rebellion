
void __thiscall FUN_004b4d90(void *this,uint *param_1)

{
  uint uVar1;
  uint uVar2;
  void *pvVar3;
  
  *param_1 = *param_1 | 0x4000000;
  pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x44),
                        (uint *)(*(int *)((int)this + 0x10) + 0x34));
  if (pvVar3 != (void *)0x0) {
    uVar1 = *param_1;
    *param_1 = uVar1 | 0x8000000;
    uVar2 = *(uint *)((int)pvVar3 + 0x2c);
    if ((((uVar2 & 8) != 0) && ((uVar2 & 0x400) != 0)) &&
       (((uVar2 & 0x400000) != 0 || ((uVar1 & 0x200000) != 0)))) {
      *param_1 = uVar1 | 0x8020000;
    }
  }
  return;
}

