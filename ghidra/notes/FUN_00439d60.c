
void __thiscall FUN_00439d60(void *this,uint param_1)

{
  undefined4 *puVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  
  iVar2 = 0;
  uVar4 = param_1;
  if (*(int *)((int)this + 0x10) != 0) {
    if (param_1 == 0x115) {
      iVar2 = FUN_005f5500((void *)((int)this + 0x18),0x15);
      param_1 = 0x100;
      uVar4 = 0x10;
    }
    else if (param_1 == 0x116) {
      iVar2 = FUN_005f5500((void *)((int)this + 0x18),0x14);
      param_1 = 0x200;
      uVar4 = 0x20;
    }
  }
  if (iVar2 != 0) {
    puVar1 = (undefined4 *)(iVar2 + 0x1c);
    iVar2 = FUN_005f4960(puVar1);
    if (iVar2 == 2) {
      *puVar1 = 0;
      uVar3 = *(int *)((int)this + 0x160) + 1;
      *(uint *)((int)this + 0x160) = uVar3;
      if (3 < uVar3) {
        DAT_006b28b0 = DAT_006b28b0 | 0x1000000;
        return;
      }
      DAT_006b28b0 = DAT_006b28b0 | uVar4;
      return;
    }
    DAT_006b28b0 = DAT_006b28b0 | param_1;
    *puVar1 = 2;
    *(int *)((int)this + 0x160) = *(int *)((int)this + 0x160) + -1;
  }
  return;
}

