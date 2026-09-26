
undefined4 __thiscall FUN_0047ba90(void *this,uint *param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 uVar4;
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006337c8;
  local_c = ExceptionList;
  bVar2 = false;
  uVar4 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(local_18);
  local_4 = 0;
  pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x68) + 0x8c),param_1);
  if (pvVar3 != (void *)0x0) {
    uVar4 = 1;
    FUN_004f26d0(local_18,(undefined4 *)((int)pvVar3 + 0x38));
    local_18[1] = 0x90;
    local_18[2] = 0x98;
    if ((0x8f < local_18[0] >> 0x18) && (local_18[0] >> 0x18 < 0x98)) {
      bVar2 = true;
    }
    FUN_00619730();
    if ((bVar2) && ((*(byte *)((int)pvVar3 + 0x30) & 0x20) == 0)) {
      iVar1 = *(int *)((int)this + 0x68);
      FUN_004f26d0(&stack0xffffffd4,local_18);
      pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar3 != (void *)0x0) && ((*(byte *)((int)pvVar3 + 0x28) & 2) != 0)) {
        uVar4 = 0;
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar4;
}

