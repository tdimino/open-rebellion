
bool FUN_0054bac0(void *param_1,void *param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined3 extraout_var;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647f6f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = (undefined4 *)FUN_00618b70(0x30);
  if (puVar2 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar2 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)0x0;
  }
  else {
    puVar2 = FUN_0058adb0(puVar2);
  }
  local_4 = 0xffffffff;
  bVar4 = puVar2 != (undefined4 *)0x0;
  if (puVar2 != (undefined4 *)0x0) {
    if ((puVar2[7] == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if (puVar2 != (undefined4 *)0x0) {
      iVar3 = FUN_0058ae90(puVar2,param_1);
      if ((iVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      bVar1 = FUN_004f4270(param_2,puVar2);
      if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

