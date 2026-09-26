
void __thiscall FUN_00488030(void *this,int *param_1)

{
  bool bVar1;
  ushort uVar2;
  short sVar3;
  int iVar4;
  int *piVar5;
  undefined2 extraout_var;
  uint uVar6;
  undefined2 extraout_var_00;
  undefined2 extraout_var_01;
  undefined2 extraout_var_02;
  uint uVar7;
  undefined1 *puVar8;
  undefined1 auStack_24 [24];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  piVar5 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00634ab0;
  pvStack_c = ExceptionList;
  uVar7 = 0;
  ExceptionList = &pvStack_c;
  iVar4 = (**(code **)(*param_1 + 0xc))();
  if ((iVar4 == 0x201) || (iVar4 == 0x240)) {
    iVar4 = (**(code **)(*piVar5 + 0x28))();
    sVar3 = FUN_005f50e0(iVar4);
    if (sVar3 == 1) {
      iVar4 = (**(code **)(*piVar5 + 0x28))();
      iVar4 = thunk_FUN_005f5060(iVar4);
      uVar6 = *(uint *)(iVar4 + 0x18) >> 0x18;
      if ((uVar6 < 0x30) || (0x37 < uVar6)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        switch(*(uint *)(iVar4 + 0x18) & 0xffffff) {
        case 0x240:
          puVar8 = (undefined1 *)0xf1;
          break;
        case 0x241:
          puVar8 = (undefined1 *)0xdd;
          break;
        case 0x242:
          iVar4 = FUN_004f3e50(*(int *)((int)this + 0x24));
          if ((*(byte *)(iVar4 + 0xb0) & 8) == 0) {
            FUN_004025b0(*(void **)(iVar4 + 0x1c),(uint *)&param_1);
            uStack_4 = 0;
            if (((uint)param_1 >> 0x18 < 0x90) || (0x97 < (uint)param_1 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if ((bVar1) && (((uint)param_1 & 0xffffff) == 0xf9)) {
              uVar2 = FUN_004c4990(&DAT_00000131);
              uVar7 = CONCAT22(extraout_var_00,uVar2);
            }
            if ((short)uVar7 == 0) {
              uVar2 = FUN_004c4990(&DAT_000000c9);
              uVar7 = CONCAT22(extraout_var_01,uVar2);
            }
            uStack_4 = 0xffffffff;
            FUN_00619730();
            goto switchD_004881ef_caseD_244;
          }
          puVar8 = &DAT_000000c9;
          break;
        case 0x243:
          puVar8 = (undefined1 *)0xe7;
          break;
        default:
          goto switchD_004881ef_caseD_244;
        case 0x280:
          puVar8 = &DAT_00000105;
          break;
        case 0x281:
          puVar8 = (undefined1 *)0xfb;
        }
        uVar2 = FUN_004c4990(puVar8);
        uVar7 = CONCAT22(extraout_var_02,uVar2);
      }
    }
    goto switchD_004881ef_caseD_244;
  }
  if (iVar4 != 0x250) goto switchD_004881ef_caseD_244;
  iVar4 = (**(code **)(*piVar5 + 0x28))();
  iVar4 = thunk_FUN_005f5060(iVar4);
  if ((iVar4 == 0) ||
     (piVar5 = FUN_004f3000(*(int *)((int)this + 0x24),(uint *)(iVar4 + 0x18)), piVar5 == (int *)0x0
     )) goto switchD_004881ef_caseD_244;
  FUN_004f26d0(&param_1,piVar5 + 0x1f);
  bVar1 = true;
  uStack_4 = 1;
  if (((uint)param_1 >> 0x18 < 0x30) || (0x37 < (uint)param_1 >> 0x18)) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    switch((uint)param_1 & 0xffffff) {
    case 0x240:
      puVar8 = (undefined1 *)0xf5;
      break;
    case 0x241:
      puVar8 = (undefined1 *)0xe1;
      break;
    case 0x242:
      FUN_004f3e50(*(int *)((int)this + 0x24));
      puVar8 = (undefined1 *)0xcd;
      break;
    case 0x243:
      puVar8 = (undefined1 *)0xeb;
      break;
    default:
      goto switchD_00488109_caseD_244;
    case 0x280:
      puVar8 = &DAT_00000109;
      break;
    case 0x281:
      puVar8 = (undefined1 *)0xff;
    }
    uVar2 = FUN_004c4990(puVar8);
    uVar7 = CONCAT22(extraout_var,uVar2);
  }
switchD_00488109_caseD_244:
  uStack_4 = 0xffffffff;
  FUN_00619730();
switchD_004881ef_caseD_244:
  if ((short)uVar7 != 0) {
    FUN_004c4a20(auStack_24,uVar7);
  }
  ExceptionList = pvStack_c;
  return;
}

