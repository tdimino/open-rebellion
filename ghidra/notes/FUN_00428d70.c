
void __thiscall FUN_00428d70(void *this,int *param_1)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  HDC pHVar4;
  HDC hdc;
  undefined4 uVar5;
  void *pvVar6;
  uint *puVar7;
  HGDIOBJ pvVar8;
  undefined *puVar9;
  char *pcVar10;
  int iVar11;
  void *local_8c;
  int *local_88;
  RECT RStack_84;
  int aiStack_74 [10];
  undefined1 auStack_4c [52];
  void *pvStack_18;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c324;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = FUN_004291d0(this,param_1[9]);
  FUN_004ece30(&local_88);
  local_4 = 0;
  (**(code **)(*param_1 + 0x68))(&local_8c,0,0);
  iStack_10._0_1_ = 1;
  local_88 = FUN_004f3220(*(int *)((int)this + 0x9c),(uint *)&stack0xffffff68);
  if (iVar2 != 0) {
    piVar1 = (int *)((int)this + (iVar2 + 0x19) * 0x1c);
    uVar3 = (**(code **)(*param_1 + 0x74))();
    RStack_84.left = piVar1[2];
    RStack_84.top = piVar1[3];
    RStack_84.right = piVar1[4];
    RStack_84.bottom = piVar1[5];
    if ((uVar3 != 0) && (uVar3 != piVar1[1])) {
      pHVar4 = GetDC(*(HWND *)((int)this + 0x18));
      hdc = CreateCompatibleDC(pHVar4);
      ReleaseDC(*(HWND *)((int)this + 0x18),pHVar4);
      FUN_00601880(auStack_4c,(char *)0x0,0,0,0,0,0,0);
      iStack_10._0_1_ = 2;
      uVar5 = FUN_006037f0(7);
      FUN_005fbd20(aiStack_74,uVar5,uVar3,2);
      iStack_10._0_1_ = 3;
      local_8c = (void *)FUN_005ff440((int)this);
      if (*piVar1 == 0) {
        pvVar6 = (void *)FUN_00618b70(0x28);
        iStack_10._0_1_ = 4;
        if (pvVar6 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pHVar4 = hdc;
          puVar7 = (uint *)FUN_005ff440((int)this);
          pvVar6 = FUN_005fbea0(pvVar6,RStack_84.right - RStack_84.left,
                                RStack_84.bottom - RStack_84.top,puVar7,pHVar4);
        }
        iStack_10._0_1_ = 3;
        *piVar1 = (int)pvVar6;
      }
      piVar1[1] = uVar3;
      FUN_005fcc30(local_8c,(uint *)*piVar1,0,0,RStack_84.left,RStack_84.top,
                   (undefined4 *)(RStack_84.right - RStack_84.left),RStack_84.bottom - RStack_84.top
                  );
      FUN_005fd0f0(aiStack_74,(uint *)*piVar1,0,0);
      pvVar8 = (HGDIOBJ)FUN_005fc130(*piVar1);
      pvVar8 = SelectObject(hdc,pvVar8);
      puVar9 = FUN_004f62d0((int)local_88);
      pcVar10 = (char *)FUN_00583c40((int)puVar9);
      FUN_00601aa0(auStack_4c,pcVar10);
      FUN_00601c60(auStack_4c,10);
      iVar11 = 0;
      iVar2 = FUN_005fc0e0(aiStack_74);
      FUN_00601b30(auStack_4c,iVar2,iVar11);
      FUN_00601c90(auStack_4c,0x200f0f0,(HWND)0x0);
      FUN_00601ce0(auStack_4c,hdc);
      SelectObject(hdc,pvVar8);
      DeleteDC(hdc);
      InvalidateRect(*(HWND *)((int)this + 0x18),&RStack_84,0);
      iStack_10._0_1_ = 2;
      FUN_005fbfa0(aiStack_74);
      iStack_10._0_1_ = 1;
      FUN_00601990((int)auStack_4c);
    }
  }
  iStack_10 = (uint)iStack_10._1_3_ << 8;
  FUN_00619730();
  iStack_10 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_18;
  return;
}

