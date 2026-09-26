
HGDIOBJ FUN_00405050(int *param_1,uint param_2,HWND param_3,uint param_4)

{
  int iVar1;
  bool bVar2;
  undefined4 uVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  HDC pHVar7;
  int *piVar8;
  HPALETTE pHVar9;
  undefined4 uVar10;
  uint uVar11;
  DWORD DVar12;
  DWORD DVar13;
  HGDIOBJ pvVar14;
  code *pcVar15;
  HWND pHVar16;
  tagPAINTSTRUCT local_4c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006297ab;
  local_c = ExceptionList;
  pHVar16 = (HWND)param_1[6];
  if (0xf < param_2) {
    if (param_2 == 0x111) {
      ExceptionList = &local_c;
      FUN_00406000(param_1,(uint)param_3 & 0xffff);
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
    if (param_2 == 0x202) {
      ExceptionList = &local_c;
      iVar6 = FUN_00601550(param_1 + 0x39,param_4 & 0xffff,param_4 >> 0x10);
      param_1[0x37] = iVar6;
      if (iVar6 == 0) {
        ExceptionList = local_c;
        return (HGDIOBJ)0x0;
      }
      iVar1 = param_1[0x2b];
      if (iVar1 == 0) {
        ExceptionList = local_c;
        return (HGDIOBJ)0x0;
      }
      iVar6 = *(int *)(iVar6 + 0x28);
      bVar2 = true;
      if (iVar6 == 0) {
        if (((byte)*(undefined4 *)(iVar1 + 0x134) & 3) == 2) {
          bVar2 = false;
        }
        else if (*(int *)(iVar1 + 0x138) != 0) {
          *(undefined4 *)(iVar1 + 0x138) = 0;
        }
      }
      else if (iVar6 == 1) {
        if (((byte)*(undefined4 *)(iVar1 + 0x134) & 3) == 1) {
          bVar2 = false;
        }
      }
      else if (iVar6 == 2) {
        if ((*(byte *)(iVar1 + 0x134) & 3) == 0) {
          bVar2 = false;
        }
        else if (*(int *)(iVar1 + 0x138) != 1) {
          *(undefined4 *)(iVar1 + 0x138) = 1;
        }
      }
      if (bVar2) {
        FUN_006030f0((void *)param_1[0x2b],0x10);
        iVar6 = param_1[0x2b];
        if ((((byte)*(undefined4 *)(iVar6 + 0x134) & 3) == 1) ||
           (*(int *)(param_1[0x37] + 0x28) == 1)) {
          SendMessageA(*(HWND *)(iVar6 + 0x18),0x201,0,0);
          pcVar15 = UpdateWindow_exref;
          UpdateWindow(*(HWND *)(param_1[0x2b] + 0x18));
          Sleep(100);
          pHVar16 = *(HWND *)(param_1[0x2b] + 0x18);
        }
        else {
          SendMessageA(*(HWND *)(iVar6 + 0x18),0x201,0,0);
          pcVar15 = UpdateWindow_exref;
          UpdateWindow(*(HWND *)(param_1[0x2b] + 0x18));
          Sleep(100);
          SendMessageA(*(HWND *)(param_1[0x2b] + 0x18),0x202,0,0);
          UpdateWindow(*(HWND *)(param_1[0x2b] + 0x18));
          SendMessageA(*(HWND *)(param_1[0x2b] + 0x18),0x201,0,0);
          UpdateWindow(*(HWND *)(param_1[0x2b] + 0x18));
          Sleep(100);
          pHVar16 = *(HWND *)(param_1[0x2b] + 0x18);
        }
        SendMessageA(pHVar16,0x202,0,0);
        (*pcVar15)(*(undefined4 *)(param_1[0x2b] + 0x18));
        FUN_006030c0((void *)param_1[0x2b],0x10);
        ExceptionList = local_c;
        return (HGDIOBJ)0x0;
      }
      param_1[0x37] = 0;
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
LAB_00405277:
    ExceptionList = &local_c;
    pvVar14 = FUN_005ff530(param_1,param_2,param_3,param_4);
    ExceptionList = local_c;
    return pvVar14;
  }
  if (param_2 != 0xf) {
    if (param_2 == 1) {
      DAT_006b1208 = 0;
      ExceptionList = &local_c;
      uVar3 = FUN_006037f0(8);
      pvVar4 = (void *)FUN_00618b70(0x28);
      local_4 = 0;
      if (pvVar4 == (void *)0x0) {
        puVar5 = (uint *)0x0;
      }
      else {
        puVar5 = FUN_005fbd20(pvVar4,uVar3,0x4e21,2);
      }
      local_4 = 0xffffffff;
      FUN_005ff2d0(param_1,puVar5,0,0);
      param_1[9] = 4;
      iVar6 = FUN_005ff440((int)param_1);
      if (iVar6 != 0) {
        pHVar7 = GetDC(pHVar16);
        piVar8 = (int *)FUN_005ff440((int)param_1);
        pHVar9 = FUN_005fc3f0(piVar8);
        FUN_00600010(pHVar7,pHVar9);
        ReleaseDC(pHVar16,pHVar7);
      }
      DAT_006be5ac = FUN_006037f0(1);
      uVar3 = *(undefined4 *)(param_1[8] + 0x18);
      uVar10 = FUN_00401a80();
      FUN_00401a60(uVar10);
      FUN_00610be0(uVar3,1,0);
      FUN_004173c0();
      uVar11 = FUN_0041d3b0();
      FUN_00417520(uVar11);
      FUN_00405560(param_1);
      ExceptionList = local_c;
      return (HGDIOBJ)0x0;
    }
    goto LAB_00405277;
  }
  ExceptionList = &local_c;
  pHVar7 = BeginPaint(pHVar16,&local_4c);
  (**(code **)(*param_1 + 0x28))(pHVar7);
  iVar6 = param_1[0x3b];
  if (iVar6 == 1) {
    if ((int *)param_1[0x36] == (int *)0x0) goto LAB_00405242;
    DVar12 = FUN_005fc0f0((int *)param_1[0x36]);
    DVar13 = FUN_005fc0e0((int *)param_1[0x36]);
    pvVar4 = (void *)param_1[0x36];
    iVar6 = 0x122;
  }
  else if (iVar6 == 2) {
    if ((int *)param_1[0x35] == (int *)0x0) goto LAB_00405242;
    DVar12 = FUN_005fc0f0((int *)param_1[0x35]);
    DVar13 = FUN_005fc0e0((int *)param_1[0x35]);
    pvVar4 = (void *)param_1[0x35];
    iVar6 = 0x146;
  }
  else {
    if ((iVar6 != 3) || ((int *)param_1[0x34] == (int *)0x0)) goto LAB_00405242;
    DVar12 = FUN_005fc0f0((int *)param_1[0x34]);
    DVar13 = FUN_005fc0e0((int *)param_1[0x34]);
    pvVar4 = (void *)param_1[0x34];
    iVar6 = 0x16a;
  }
  FUN_005fc140(pvVar4,pHVar7,'@',0xcc0020,iVar6,0x125,0,0,DVar13,DVar12);
LAB_00405242:
  if ((void *)param_1[0x28] != (void *)0x0) {
    FUN_00601ce0((void *)param_1[0x28],pHVar7);
  }
  EndPaint(pHVar16,&local_4c);
  ExceptionList = local_c;
  return (HGDIOBJ)0x0;
}

