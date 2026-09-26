
uint __fastcall FUN_00519350(int *param_1)

{
  bool bVar1;
  uint uVar2;
  void *this;
  uint *puVar3;
  void *this_00;
  undefined3 extraout_var;
  uint local_1c;
  uint local_18;
  uint local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642c08;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_004f9700(param_1);
  this = (void *)FUN_00505340();
  while ((uVar2 != 0 && (this != (void *)0x0))) {
    if (((*(uint *)((int)this + 0x50) & 2) != 0) && ((*(uint *)((int)this + 0x50) & 0x1c) == 0)) {
      FUN_004f26d0(&local_1c,(undefined4 *)((int)this + 0x38));
      local_4 = 1;
      puVar3 = FUN_004ece40(&local_1c);
      if (puVar3 != (uint *)0x0) {
        this_00 = (void *)FUN_00505750(&local_1c);
        uVar2 = 0;
        if (this_00 != (void *)0x0) {
          puVar3 = FUN_004025b0(this,&local_18);
          local_4._0_1_ = 2;
          bVar1 = FUN_00528930(this_00,puVar3);
          uVar2 = CONCAT31(extraout_var,bVar1);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
        }
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    local_14 = 0x10;
    local_10 = 0x40;
    local_4 = 0;
    this = (void *)FUN_004f6010(this,&local_14,1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar2;
}

