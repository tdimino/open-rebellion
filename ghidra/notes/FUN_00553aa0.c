
undefined4 __cdecl FUN_00553aa0(int param_1,void *param_2,int *param_3)

{
  bool bVar1;
  void *pvVar2;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006490d0;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0x25;
  local_4 = 0;
  if ((*param_3 != 1) || (ExceptionList = &local_c, param_3[1] != 0x25)) {
    local_1c = 1;
    local_18 = 0x27;
    if ((*param_3 == 1) && (param_3[1] == 0x27)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (!bVar1) {
      bVar1 = false;
      goto LAB_00553b14;
    }
  }
  bVar1 = true;
LAB_00553b14:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    pvVar2 = FUN_004f5940(param_2,(uint *)(param_1 + 8));
    if (pvVar2 != (void *)0x0) {
      local_1c = 1;
      local_18 = 0x26;
      local_4 = 1;
      FUN_00520580(param_3,&local_1c);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return 1;
}

