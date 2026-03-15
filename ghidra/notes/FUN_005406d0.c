
bool __fastcall FUN_005406d0(int *param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00646ac8;
  pvStack_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  uVar2 = (**(code **)(*param_1 + 4))();
  if ((uVar2 < 0xf1) || (0xf1 < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return !bVar1;
}

