
bool __thiscall FUN_0052e970(void *this,uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  bool bVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006452a0;
  local_c = ExceptionList;
  bVar4 = true;
  local_14 = 1;
  local_10 = 0xffffffff;
  bVar1 = false;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x10;
  local_10 = 0x40;
  if ((0xf < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x40)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0051cab0(param_1);
    bVar4 = iVar2 != 0;
    if (bVar4) {
      iVar2 = FUN_0053b870(iVar2);
      iVar2 = iVar2 * param_2;
      if ((iVar2 != 0) &&
         (iVar3 = *(int *)((int)this + 0x58) - *(int *)((int)this + 0x5c),
         iVar2 - iVar3 != 0 && iVar3 <= iVar2)) {
        local_14 = 0xf3;
        local_10 = 1;
        local_4 = 1;
        FUN_00520580(param_3,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

