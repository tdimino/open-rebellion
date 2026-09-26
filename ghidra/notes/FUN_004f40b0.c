
uint __thiscall FUN_004f40b0(void *this,void *param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  int local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f667;
  local_c = ExceptionList;
  uVar4 = 1;
  ExceptionList = &local_c;
  FUN_0042dbe0(&local_14);
  local_4 = 0;
  FUN_005f5540(this);
  FUN_005f4d90(param_1,(int)this + 0xc);
  FUN_005f4d90(param_1,&local_18);
  local_1c = 0;
  if (0 < local_18) {
    do {
      puVar2 = (undefined4 *)FUN_00618b70(0x28);
      if (puVar2 == (undefined4 *)0x0) {
        FUN_00619730();
      }
      local_4._0_1_ = 1;
      local_10 = puVar2;
      if (puVar2 == (undefined4 *)0x0) {
        puVar2 = (undefined4 *)0x0;
      }
      else {
        puVar2 = FUN_00540a50(puVar2);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      uVar4 = 0;
      if (puVar2 != (undefined4 *)0x0) {
        iVar3 = FUN_00540b80(puVar2,param_1);
        uVar4 = 0;
        if (iVar3 != 0) {
          bVar1 = FUN_004f4270(this,puVar2);
          uVar4 = CONCAT31(extraout_var,bVar1);
        }
      }
      local_1c = local_1c + 1;
    } while ((uVar4 & local_1c < local_18) != 0);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar4;
}

