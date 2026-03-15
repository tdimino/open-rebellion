
UINT FUN_0040ec60(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  POINT pt;
  POINT pt_00;
  int iVar1;
  undefined4 *puVar2;
  int *piVar3;
  BOOL BVar4;
  UINT UVar5;
  uint uVar6;
  undefined4 local_28;
  int local_24;
  uint local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a3a7;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (((param_1[0x45] == 1) &&
      (ExceptionList = &local_c, FUN_004104b0(param_1), param_1[0x4b] == 0x2777)) &&
     (param_1[0x47] == 1)) {
    FUN_004115c0((int)param_1);
  }
  if (param_2 < 0x202) {
    if (param_2 == 0x201) {
      iVar1 = (int)(short)((uint)param_4 >> 0x10);
      if (param_1[0x4b] == 0x2774) {
        ExceptionList = local_c;
        return 0;
      }
      if (param_1[0x4b] != 0x2777) {
        ExceptionList = local_c;
        return 0;
      }
      if (param_1[0x47] == 0) {
        ExceptionList = local_c;
        return 0;
      }
      pt.y = iVar1;
      pt.x = (int)(short)param_4;
      BVar4 = PtInRect((RECT *)&lprc_006586a8,pt);
      if (BVar4 != 0) {
        FUN_0040f9f0(param_1,1,0,1);
        ExceptionList = local_c;
        return 0;
      }
      pt_00.y = iVar1;
      pt_00.x = (int)(short)param_4;
      BVar4 = PtInRect((RECT *)&lprc_006586b8,pt_00);
      if (BVar4 == 0) {
        ExceptionList = local_c;
        return 0;
      }
      FUN_0040f9f0(param_1,2,0,1);
      ExceptionList = local_c;
      return 0;
    }
    if (param_2 == 0x113) {
      if (param_3 == (HPALETTE)0x190) {
        if (param_1[0x4b] != 0x2777) {
          ExceptionList = local_c;
          return 0;
        }
        if (((byte)DAT_006b123c & 1) != 0) {
          ExceptionList = local_c;
          return 0;
        }
        if (param_1[0x47] == 0) {
          ExceptionList = local_c;
          return 0;
        }
        puVar2 = (undefined4 *)FUN_00618b70(0x3c);
        if (puVar2 == (undefined4 *)0x0) {
          FUN_00619730();
        }
        local_4 = 0;
        if (puVar2 == (undefined4 *)0x0) {
          piVar3 = (int *)0x0;
        }
        else {
          piVar3 = FUN_00420bd0(puVar2);
        }
        local_4 = 0xffffffff;
        if (piVar3 == (int *)0x0) {
          ExceptionList = local_c;
          return 0;
        }
        FUN_0051c300(&local_28);
        local_4 = 1;
        FUN_0051c410(&local_28,piVar3);
        if (local_24 != 0) {
          FUN_005970d0(local_10,local_14,1);
          FUN_00597170();
        }
        (**(code **)*piVar3)(1);
        local_4 = 0xffffffff;
        FUN_0051c350(&local_28);
        ExceptionList = local_c;
        return 0;
      }
      if (param_3 != (HPALETTE)0x191) {
        ExceptionList = local_c;
        return 0;
      }
      if ((param_1[100] != 0) || (param_1[0x4b] != 0x2775)) {
        KillTimer((HWND)param_1[6],0x191);
        ExceptionList = local_c;
        return 0;
      }
      if (param_1[0x46] != 0) {
        ExceptionList = local_c;
        return 0;
      }
      iVar1 = FUN_00597040();
      if (iVar1 != 0) {
        ExceptionList = local_c;
        return 0;
      }
      FUN_0040f450(param_1);
      ExceptionList = local_c;
      return 0;
    }
LAB_0040ee76:
    UVar5 = FUN_00606650(param_1,param_2,param_3,param_4);
    ExceptionList = local_c;
    return UVar5;
  }
  if (param_2 == 0x407) {
    FUN_00410240(param_1,0);
    ExceptionList = local_c;
    return 0;
  }
  if (param_2 != 0x40c) goto LAB_0040ee76;
  switch(param_3) {
  case (HPALETTE)0x1:
  case (HPALETTE)0x2:
    FUN_004109e0((int)param_1);
    ExceptionList = local_c;
    return 0;
  case (HPALETTE)0x3:
    FUN_004109e0((int)param_1);
    break;
  case (HPALETTE)0x4:
    uVar6 = FUN_00597090();
    if (1 < uVar6) {
      ExceptionList = local_c;
      return 0;
    }
    break;
  default:
    goto switchD_0040ee9e_default;
  }
  FUN_004109e0((int)param_1);
  FUN_005974c0();
  FUN_00597050();
  (**(code **)(*param_1 + 0x38))();
  iVar1 = FUN_00597480();
  if (iVar1 == 0) {
    FUN_00410b10(param_1);
  }
  else {
    FUN_00597340();
    FUN_00597360();
    param_1[0x46] = 1;
    KillTimer((HWND)param_1[6],0x191);
    KillTimer((HWND)param_1[6],400);
    FUN_00597050();
    DAT_006b1208 = 0;
    iVar1 = FUN_00597480();
    param_1[0x46] = 1;
    if (iVar1 == 0) {
      FUN_00401040(3);
    }
    else {
      FUN_00401040(0xe);
    }
  }
  FUN_00401150(0x46e,0,0x1205,0x190f);
switchD_0040ee9e_default:
  ExceptionList = local_c;
  return 0;
}

