
bool FUN_005438a0(int *param_1)

{
  int iVar1;
  bool bVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00647338;
  local_c = ExceptionList;
  local_4 = 0;
  iVar1 = *param_1;
  ExceptionList = &local_c;
  if (iVar1 != 0x32000242) {
    ExceptionList = &local_c;
    FUN_00619730();
    if (iVar1 != 0x31000241) {
      iVar1 = *param_1;
      FUN_00619730();
      if (iVar1 != 0x33000243) {
        iVar1 = *param_1;
        FUN_00619730();
        if (iVar1 != 0x38000343) {
          iVar1 = *param_1;
          FUN_00619730();
          if (iVar1 != 0x35000281) {
            bVar2 = *param_1 == 0x34000280;
            FUN_00619730();
            if (!bVar2) goto LAB_00543985;
          }
        }
      }
    }
  }
  bVar2 = true;
LAB_00543985:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar2;
}

