
bool __thiscall FUN_00508c60(void *param_1,int *param_2,int param_3)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  bool bVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00641848;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = FUN_00524b10();
  bVar6 = iVar2 != 0;
  if ((param_3 == 0) && ((*(byte *)(param_2 + 0x14) & 8) != 0)) {
    iStack_4 = param_3;
    uVar3 = (**(code **)(*param_2 + 4))();
    if ((uVar3 < 0x10) || (0x1f < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iStack_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      iVar2 = (**(code **)(*param_2 + 0x1c0))();
      iVar4 = (**(code **)(*param_2 + 0x1c8))();
      iVar2 = iVar2 - iVar4;
      uVar5 = param_2[0x10] & 0xff;
      uVar3 = (uint)param_2[9] >> 6 & 3;
      if (uVar5 == 0xc) {
        iVar4 = FUN_00509ae0(param_1,uVar3);
        iVar2 = FUN_0050cee0(param_1,iVar4 + iVar2,uVar3);
        if ((iVar2 != 0) && (bVar6)) {
          ExceptionList = pvStack_c;
          return true;
        }
      }
      else {
        if (uVar5 == 0xe) {
          iVar4 = FUN_005099c0(param_1,uVar3);
          iVar2 = FUN_0050ce20(param_1,iVar4 + iVar2,uVar3);
        }
        else {
          if (uVar5 != 0xf) {
            ExceptionList = pvStack_c;
            return bVar6;
          }
          iVar4 = FUN_005099f0(param_1,uVar3);
          iVar2 = FUN_0050ce50(param_1,iVar4 + iVar2,uVar3);
        }
        if ((iVar2 != 0) && (bVar6)) {
          ExceptionList = pvStack_c;
          return true;
        }
      }
      bVar6 = false;
    }
  }
  ExceptionList = pvStack_c;
  return bVar6;
}

