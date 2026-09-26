
bool __thiscall FUN_004feb60(void *this,undefined4 param_1)

{
  int iVar1;
  int unaff_ESI;
  bool bVar2;
  void *unaff_retaddr;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006405a8;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*(int *)this + 0x1c0))(param_1,&local_14);
  bVar2 = iVar1 != 0;
  if (unaff_ESI == -1) {
    iVar1 = FUN_004fe4d0(this,1,unaff_retaddr);
    if ((iVar1 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
  }
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return bVar2;
}

