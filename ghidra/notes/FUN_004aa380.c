
void * __thiscall FUN_004aa380(void *this,void *param_1,undefined4 param_2,undefined4 param_3)

{
  short sVar1;
  undefined4 *puVar2;
  void *this_00;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006382c7;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004f26d0(&local_14,(undefined4 *)((int)this + 0x144));
  local_4 = 1;
  sVar1 = (short)param_2;
  param_2 = CONCAT22((short)param_3,sVar1);
  if (*(void **)((int)this + 0x164) != (void *)0x0) {
    param_2 = CONCAT22((short)param_3 + -0x51,sVar1 + -7);
    puVar2 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x164),&param_2,param_2);
    this_00 = (void *)FUN_00609d10(*(void **)((int)this + 0x164),*puVar2);
    if (this_00 != (void *)0x0) {
      puVar2 = FUN_0042d170(this_00,&param_2);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_14,puVar2);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
    }
  }
  FUN_004f26d0(param_1,&local_14);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

