
undefined4 * __fastcall FUN_005f9860(int *param_1)

{
  int *piVar1;
  char cVar2;
  DWORD DVar3;
  undefined4 *puVar4;
  uint uVar5;
  int iVar6;
  int *piVar7;
  uint uVar8;
  uint uVar9;
  void *pvVar10;
  undefined4 *puVar11;
  int iStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 *puStack_e0;
  undefined4 auStack_dc [3];
  undefined4 auStack_d0 [3];
  undefined4 auStack_c4 [3];
  undefined4 auStack_b8 [3];
  undefined4 auStack_ac [3];
  undefined4 auStack_a0 [3];
  undefined4 auStack_94 [3];
  undefined4 auStack_88 [3];
  undefined4 auStack_7c [3];
  undefined1 auStack_70 [100];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655cbc;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  DVar3 = timeGetTime();
  if ((uint)param_1[0x25] < 2) {
    param_1[0x34] = DVar3;
    param_1[0x35] = 0;
  }
  else if (29999 < DVar3 - param_1[0x34]) {
    if (param_1[0x35] == 0) {
      FUN_005fa400(param_1);
    }
    else if (89999 < DVar3 - param_1[0x34]) {
      FUN_005f8600(param_1);
      FUN_005ae1e0();
      PostMessageA((HWND)param_1[0xb],0x40c,3,0);
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
  }
  if (param_1[0x42] == 0) {
    ExceptionList = pvStack_c;
    return (undefined4 *)0x0;
  }
LAB_005f9900:
  do {
    while( true ) {
      piVar1 = param_1 + 0x91f;
      cVar2 = FUN_005fb0c0((int)piVar1);
      if (cVar2 == '\0') break;
      puVar4 = (undefined4 *)FUN_005fb180(piVar1);
      uVar8 = puVar4[8];
      uVar5 = FUN_005f88c0((int)param_1);
      if (uVar8 >> 0x1c == uVar5) {
        FUN_005f8eb0(param_1,(int)puVar4,param_1[0x9a2]);
      }
      else {
        if ((*(char *)((int)puVar4 + 0x26) != '\0') && (puVar4[8] != 0)) {
          FUN_005f93d0(param_1,(int)puVar4);
          FUN_005f9030(param_1);
        }
        uVar8 = puVar4[8];
        if ((uVar8 & 0xffffff) != 0xffffff) {
          iVar6 = FUN_005f5500(param_1 + 6,uVar8);
          if (iVar6 == 0) {
            puStack_e0 = (undefined4 *)FUN_00618b70(0x1c);
            puVar11 = (undefined4 *)0x0;
            uStack_4 = 0;
            if (puStack_e0 != (undefined4 *)0x0) {
              puVar11 = FUN_005f5590(puStack_e0);
            }
            uStack_4 = 0xffffffff;
            FUN_005f5600(puVar11,uVar8);
            FUN_005f5440(param_1 + 6,puVar11);
            FUN_00618ed0(auStack_70,s_AckID__u__Category__u_006ac58c);
            ExceptionList = pvStack_c;
            return puVar4;
          }
          goto LAB_005f9b30;
        }
      }
LAB_005f9b1b:
      if (puVar4 != (undefined4 *)0x0) {
        (**(code **)*puVar4)(1);
      }
    }
    (**(code **)*piVar1)();
    if (param_1[0x42] == 0) {
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
    iStack_ec = 0;
    uStack_e4 = 0;
    uStack_e8 = 0x200;
    piVar7 = (int *)FUN_005fb170((int)piVar1);
    iVar6 = (**(code **)(*(int *)param_1[0x42] + 100))
                      ((int *)param_1[0x42],&iStack_ec,&uStack_e4,1,piVar7,&uStack_e8);
    if (iVar6 != 0) {
      if (iVar6 != -0x7788ff42) {
        FUN_005f2f50(auStack_dc);
        uStack_4 = 2;
        FUN_005f35b0(auStack_88,&DAT_006a8170);
        uStack_4._0_1_ = 3;
        FUN_005f35b0(auStack_a0,s_BufferSize___006ac538);
        uStack_4._0_1_ = 4;
        FUN_005f35b0(auStack_b8,&DAT_006a8170);
        uStack_4._0_1_ = 5;
        FUN_005f35b0(auStack_c4,s_To___006ac52c);
        uStack_4._0_1_ = 6;
        FUN_005f35b0(auStack_7c,&DAT_006a8170);
        uStack_4._0_1_ = 7;
        FUN_005f35b0(auStack_ac,s_From___006ac520);
        uStack_4._0_1_ = 8;
        FUN_005f35b0(auStack_d0,&DAT_006a8170);
        uStack_4._0_1_ = 9;
        FUN_005f35b0(auStack_94,s_Direct_Player_Receive_Error___006ac500);
        uStack_4._0_1_ = 10;
        FUN_005f30d0(auStack_dc,(int)auStack_94);
        pvVar10 = FUN_005f31f0(auStack_dc,iVar6);
        FUN_005f30d0(pvVar10,(int)auStack_d0);
        FUN_005f30d0(pvVar10,(int)auStack_ac);
        pvVar10 = FUN_005f31f0(pvVar10,iStack_ec);
        FUN_005f30d0(pvVar10,(int)auStack_7c);
        FUN_005f30d0(pvVar10,(int)auStack_c4);
        pvVar10 = FUN_005f31f0(pvVar10,uStack_e4);
        FUN_005f30d0(pvVar10,(int)auStack_b8);
        FUN_005f30d0(pvVar10,(int)auStack_a0);
        pvVar10 = FUN_005f31f0(pvVar10,uStack_e8);
        FUN_005f30d0(pvVar10,(int)auStack_88);
        uStack_4._0_1_ = 9;
        FUN_005f2ff0(auStack_94);
        uStack_4._0_1_ = 8;
        FUN_005f2ff0(auStack_d0);
        uStack_4._0_1_ = 7;
        FUN_005f2ff0(auStack_ac);
        uStack_4._0_1_ = 6;
        FUN_005f2ff0(auStack_7c);
        uStack_4._0_1_ = 5;
        FUN_005f2ff0(auStack_c4);
        uStack_4._0_1_ = 4;
        FUN_005f2ff0(auStack_b8);
        uStack_4._0_1_ = 3;
        FUN_005f2ff0(auStack_a0);
        uStack_4 = CONCAT31(uStack_4._1_3_,2);
        FUN_005f2ff0(auStack_88);
        FUN_00583c40((int)auStack_dc);
        FUN_005ae1e0();
        uStack_4 = 0xffffffff;
        FUN_005f2ff0(auStack_dc);
        ExceptionList = pvStack_c;
        return (undefined4 *)0x0;
      }
      DVar3 = GetTickCount();
      if (DVar3 <= (uint)param_1[0x2b]) {
        ExceptionList = pvStack_c;
        return (undefined4 *)0x0;
      }
      FUN_00619730();
      ExceptionList = pvStack_c;
      return (undefined4 *)0x0;
    }
    DVar3 = timeGetTime();
    param_1[0x34] = DVar3;
    param_1[0x35] = 0;
    if (iStack_ec == 0) {
      if (*piVar7 == 3) {
        FUN_005ae1e0();
        FUN_005f8a30(param_1);
      }
      else if (*piVar7 == 5) {
        FUN_005ae1e0();
        uVar8 = FUN_005f9000(param_1,piVar7[1]);
        FUN_005f9790(param_1,uVar8);
        uVar8 = FUN_005f8a30(param_1);
        if (uVar8 < 2) {
          FUN_005f8600(param_1);
          FUN_005ae1e0();
          PostMessageA((HWND)param_1[0xb],0x40c,4,0);
          ExceptionList = pvStack_c;
          return (undefined4 *)0x0;
        }
        (**(code **)*piVar1)();
        goto LAB_005f9900;
      }
      (**(code **)*piVar1)();
      goto LAB_005f9900;
    }
    if (iStack_ec == param_1[0x44]) {
      (**(code **)*piVar1)();
    }
    else {
      uVar8 = FUN_005f9000(param_1,iStack_ec);
      if ((param_1[10] == 1) && (uVar8 == 0xffffffff)) {
        FUN_005f8a30(param_1);
        uVar8 = FUN_005f9000(param_1,iStack_ec);
      }
      param_1[0x9a2] = uVar8;
      puVar4 = (undefined4 *)FUN_005fb180(param_1 + 0x91f);
      uVar5 = puVar4[8];
      uVar9 = FUN_005f88c0((int)param_1);
      if (uVar5 >> 0x1c == uVar9) {
        FUN_005f8eb0(param_1,(int)puVar4,param_1[0x9a2]);
      }
      else if (uVar8 == (uint)puVar4[8] >> 0x1c) {
        if ((*(char *)((int)puVar4 + 0x26) != '\0') && (puVar4[8] != 0)) {
          FUN_005f93d0(param_1,(int)puVar4);
          FUN_005f9030(param_1);
        }
        uVar8 = puVar4[8];
        if ((uVar8 & 0xffffff) != 0xffffff) {
          iVar6 = FUN_005f5500(param_1 + 6,uVar8);
          if (iVar6 == 0) {
            puStack_e0 = (undefined4 *)FUN_00618b70(0x1c);
            uStack_4 = 1;
            if (puStack_e0 == (undefined4 *)0x0) {
              puVar11 = (undefined4 *)0x0;
            }
            else {
              puVar11 = FUN_005f5590(puStack_e0);
            }
            uStack_4 = 0xffffffff;
            FUN_005f5600(puVar11,uVar8);
            FUN_005f5440(param_1 + 6,puVar11);
            FUN_00618ed0(auStack_70,s_AckID__u__Category__u_006ac58c);
            ExceptionList = pvStack_c;
            return puVar4;
          }
          goto LAB_005f9b1b;
        }
      }
LAB_005f9b30:
      if (puVar4 != (undefined4 *)0x0) {
        (**(code **)*puVar4)(1);
      }
    }
  } while( true );
}

