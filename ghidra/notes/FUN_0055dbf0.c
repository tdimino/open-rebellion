
bool __cdecl
FUN_0055dbf0(int param_1,int param_2,int param_3,int param_4,int param_5,undefined4 *param_6)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int local_30;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  iVar4 = param_2;
  iVar8 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649e18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = FUN_00562ae0(local_28);
  iVar10 = 0;
  local_4 = 0;
  FUN_00562b80(param_6,puVar2);
  local_4 = 0xffffffff;
  FUN_00562b30();
  if ((param_3 < 1) || (bVar1 = true, 3 < param_3)) {
    bVar1 = false;
  }
  bVar1 = bVar1 && param_3 != 3;
  if (bVar1) {
    *param_6 = 0;
    FUN_0055de00((int)param_6);
    iVar7 = (param_1 <= param_2) + 1;
    if (iVar7 == 1) {
      param_1 = DAT_006bb6fc;
      param_2 = DAT_006bb634;
      local_30 = param_4;
      iVar3 = iVar4;
      iVar6 = DAT_006bb6f0;
      iVar9 = DAT_006bb5dc;
      iVar4 = iVar8;
    }
    else {
      param_2 = DAT_006bb6c8;
      param_1 = DAT_006bb62c;
      local_30 = param_5;
      iVar3 = iVar8;
      iVar6 = DAT_006bb6e0;
      param_5 = param_4;
      iVar9 = DAT_006bb678;
    }
    iVar8 = 0;
    if (iVar4 != 0) {
      iVar10 = FUN_0053e170(iVar6,iVar4 - iVar3,iVar4);
      iVar10 = iVar10 + param_1;
      iVar8 = FUN_0053e170(iVar9,iVar4 - iVar3,iVar4);
      iVar8 = (iVar9 + param_2) - iVar8;
    }
    iVar10 = FUN_0053e190(local_30,iVar10);
    iVar4 = FUN_0053e190(param_5,iVar8);
    iVar8 = iVar10;
    if (iVar7 == param_3) {
      iVar8 = iVar4;
      iVar4 = iVar10;
    }
    iVar8 = (iVar8 * 2 - iVar4) / DAT_006bb5f0;
    if (iVar8 < 0) {
      iVar8 = -iVar8;
      if (param_3 == 3) {
        param_3 = 3;
      }
      else {
        param_3 = 2 - (uint)(param_3 != 1);
      }
    }
    iVar4 = DAT_006bb6ec;
    if (param_3 != 1) {
      iVar4 = DAT_006bb690;
    }
    uVar5 = FUN_0053e0f0(iVar8,DAT_00661a84,iVar4);
    param_6[5] = param_3;
    param_6[6] = uVar5;
    if (param_3 == 3) {
      iVar8 = 3;
    }
    else {
      iVar8 = 2 - (uint)(param_3 != 1);
    }
    uVar5 = DAT_006bb5e0;
    if (iVar8 != 1) {
      uVar5 = DAT_006bb604;
    }
    param_6[3] = iVar8;
    param_6[4] = uVar5;
  }
  ExceptionList = local_c;
  return bVar1;
}

