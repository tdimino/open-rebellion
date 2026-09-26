
void __thiscall FUN_00420200(void *this,int param_1)

{
  void *this_00;
  void *this_01;
  bool bVar1;
  LPCSTR pCVar2;
  int iVar3;
  int iVar4;
  int c;
  tagSIZE *ptVar5;
  int local_3c;
  int local_34;
  tagSIZE local_2c;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b908;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f3d80(local_24);
  iVar3 = *(int *)((int)this + 300);
  this_00 = (void *)((int)this + 0x10c);
  local_4 = 0;
  FUN_005f3e30(this_00,(int)this + 0xa0);
  local_3c = *(int *)((int)this + 0x174);
  iVar4 = 0;
  if (0 < local_3c) {
    do {
      FUN_005f3e70(this_00,local_18);
      FUN_005f3e00(local_18);
      iVar4 = iVar4 + 1;
    } while (iVar4 < *(int *)((int)this + 0x174));
  }
  this_01 = (void *)((int)this + 0x100);
  FUN_005f3e10(this_01,(int)this_00);
  FUN_005f3e10(local_24,(int)this_00);
  bVar1 = false;
  iVar4 = FUN_005f3fa0((int)this_00);
  while ((iVar4 != 0 && (!bVar1))) {
    iVar4 = FUN_00420930((int)this_01,this_00);
    bVar1 = false;
    if (iVar4 == 0) {
      local_3c = local_3c + 1;
      iVar4 = FUN_005f3f80((int)this_01);
      if (iVar4 == 10) {
        FUN_00420450(this,*(undefined4 *)((int)this + 0x174));
        *(int *)((int)this + 0x174) = local_3c;
      }
      else {
        local_34 = iVar3;
        if ((iVar4 != 0x20) ||
           (local_34 = *(int *)((int)this + 0x128) + iVar3, local_34 <= *(int *)((int)this + 0x130))
           ) goto LAB_0042037d;
        FUN_00420450(this,*(undefined4 *)((int)this + 0x174));
        *(int *)((int)this + 0x174) = local_3c;
      }
      bVar1 = true;
      local_34 = iVar3;
    }
    else {
      ptVar5 = &local_2c;
      c = iVar4;
      pCVar2 = (LPCSTR)FUN_00583c40((int)this_01);
      GetTextExtentPoint32A(*(HDC *)((int)this + 0x11c),pCVar2,c,ptVar5);
      if (*(int *)((int)this + 0x130) < local_2c.cx + iVar3) {
        if (*(int *)((int)this + 0x130) - *(int *)((int)this + 300) < local_2c.cx)
        goto LAB_004203af;
        goto LAB_004203fd;
      }
      local_3c = local_3c + iVar4;
      local_34 = local_2c.cx + iVar3;
    }
LAB_0042037d:
    iVar3 = FUN_005f3e50((int)this_00);
    FUN_005f3e10(this_01,iVar3);
    iVar4 = FUN_005f3fa0((int)this_00);
    iVar3 = local_34;
  }
  goto LAB_00420415;
  while( true ) {
    iVar4 = iVar4 + -1;
    FUN_005f3ef0(this_00,local_18);
    FUN_005f3e00(local_18);
    ptVar5 = &local_2c;
    iVar3 = iVar4;
    pCVar2 = (LPCSTR)FUN_00583c40((int)this_01);
    GetTextExtentPoint32A(*(HDC *)((int)this + 0x11c),pCVar2,iVar3,ptVar5);
    if (local_2c.cx <= *(int *)((int)this + 0x130) - *(int *)((int)this + 300)) break;
LAB_004203af:
    if (iVar4 < 2) break;
  }
LAB_004203fd:
  FUN_00420450(this,*(undefined4 *)((int)this + 0x174));
  *(int *)((int)this + 0x174) = local_3c;
LAB_00420415:
  if (param_1 != 0) {
    FUN_0041fd00((int)this);
  }
  local_4 = 0xffffffff;
  FUN_005f3e00(local_24);
  ExceptionList = pvStack_c;
  return;
}

