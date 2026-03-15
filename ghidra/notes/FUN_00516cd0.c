
int __fastcall FUN_00516cd0(int param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  void *pvVar2;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642828;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_005197f0(param_1);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_004ece80((undefined4 *)(param_1 + 0x8c));
    pvVar2 = (void *)FUN_004fd400(local_28);
    local_4 = 0;
    FUN_004fd490((void *)(param_1 + 0x90),pvVar2);
    local_4 = 0xffffffff;
    FUN_004fd4d0(local_28);
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar1);
}

