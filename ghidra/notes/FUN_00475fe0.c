
undefined4 __thiscall FUN_00475fe0(void *this,uint *param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  uint uVar4;
  undefined4 uVar5;
  
  uVar5 = 0;
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)((int)this + 0x4c);
    FUN_004f26d0(&stack0xffffffe4,param_1);
    pvVar3 = FUN_00419e40((void *)(iVar1 + 0x78));
    if (pvVar3 != (void *)0x0) {
      FUN_004f26d0((void *)((int)this + 0x2c),param_1);
      uVar5 = 1;
      uVar4 = *(uint *)((int)pvVar3 + 0x38) & 0xfffffffb;
      *(uint *)((int)pvVar3 + 0x38) = uVar4;
      *(uint *)((int)pvVar3 + 0x38) = uVar4 | 8;
      *(undefined4 *)((int)pvVar3 + 0x30) = 0;
      *(undefined4 *)((int)pvVar3 + 0x34) = 0;
    }
  }
  return uVar5;
}

