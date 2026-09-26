
undefined4 __thiscall FUN_00478fd0(void *this,uint *param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  undefined4 local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063332b;
  local_c = ExceptionList;
  local_18 = 0;
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar2) {
    pvVar3 = (void *)FUN_00618b70(0x1c);
    local_4 = 0;
    if (pvVar3 == (void *)0x0) {
      puVar4 = (undefined4 *)0x0;
    }
    else {
      puVar4 = FUN_004ecc70(pvVar3,param_1);
    }
    local_4 = 0xffffffff;
    if (puVar4 != (undefined4 *)0x0) {
      iVar1 = *(int *)((int)this + 0x58);
      FUN_004f26d0(&stack0xffffffd4,param_1);
      pvVar3 = FUN_00419e40((void *)(iVar1 + 0x78));
      if (pvVar3 == (void *)0x0) {
        (**(code **)*puVar4)();
      }
      else {
        *(uint *)((int)pvVar3 + 0x38) = *(uint *)((int)pvVar3 + 0x38) & 0xfffffffb;
        *(undefined4 *)((int)pvVar3 + 0x30) = *(undefined4 *)((int)this + 0x18);
        *(undefined4 *)((int)pvVar3 + 0x34) = *(undefined4 *)((int)this + 0x44);
        FUN_004f57b0((void *)((int)this + 0x2c),puVar4);
        local_18 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return local_18;
}

