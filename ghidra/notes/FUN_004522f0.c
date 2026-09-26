
void __fastcall FUN_004522f0(void *param_1)

{
  int iVar1;
  HMODULE pHVar2;
  void *pvVar3;
  undefined4 uVar4;
  uint *puVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int *piVar9;
  void *pvVar10;
  undefined4 uVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fd11;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pHVar2 = (HMODULE)FUN_006037f0(7);
  pvVar3 = (void *)FUN_00618b70(0x28);
  piVar9 = (int *)0x0;
  local_4 = 0;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_005fbda0(pvVar3,*(int *)((int)param_1 + 0x38),*(int *)((int)param_1 + 0x3c),
                          *(uint **)(*(int *)((int)param_1 + 0x11c) + 0x45c));
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0xa4) = pvVar3;
  uVar4 = FUN_005fddd0(pvVar3,&LAB_004ac550);
  *(undefined4 *)((int)param_1 + 0x140) = uVar4;
  switch(*(undefined4 *)((int)param_1 + 0x114)) {
  case 0x11:
    if (*(int *)(*(int *)((int)param_1 + 0x11c) + 0x9c) == 1) {
      pvVar3 = (void *)0x27c7;
    }
    else {
      pvVar3 = (void *)0x27c8;
    }
    break;
  case 0x12:
    pvVar3 = (void *)0x2d50;
    break;
  default:
    goto switchD_00452389_caseD_13;
  case 0x21:
    pvVar3 = (void *)0x2d51;
    break;
  case 0x22:
    pvVar3 = (void *)0x2d52;
    break;
  case 0x30:
    pvVar3 = (void *)0x27d9;
    break;
  case 0x43:
    pvVar3 = (void *)0x2d53;
    break;
  case 0x44:
    pvVar3 = (void *)0x2d54;
    break;
  case 0x51:
    pvVar3 = (void *)0x27cc;
    break;
  case 0x52:
    pvVar3 = (void *)0x27cd;
    break;
  case 0x53:
    pvVar3 = (void *)0x27ce;
    break;
  case 0x54:
    pvVar3 = (void *)0x27cf;
    break;
  case 0x61:
    pvVar3 = (void *)0x27d0;
    break;
  case 0x62:
    pvVar3 = (void *)0x27d1;
    break;
  case 99:
    pvVar3 = (void *)0x27d2;
    break;
  case 100:
    pvVar3 = (void *)0x27d3;
    break;
  case 0x65:
    pvVar3 = (void *)0x2d55;
    break;
  case 0x66:
    pvVar3 = (void *)0x2d56;
    break;
  case 0x67:
    pvVar3 = (void *)0x2d57;
    break;
  case 0x71:
    pvVar3 = (void *)0x27d4;
    break;
  case 0x72:
    pvVar3 = (void *)0x27d5;
    break;
  case 0x73:
    pvVar3 = (void *)0x27d6;
    break;
  case 0x74:
    pvVar3 = (void *)0x27d7;
    break;
  case 0x75:
    pvVar3 = (void *)0x27d8;
  }
  FUN_00452630(param_1,pHVar2,pvVar3);
  FUN_00607740(param_1,(int *)pHVar2,0x2779,0x277a,0x277b,(int *)0x2778,(int *)0x2775,(int *)0x2774,
               0x2777,(int *)0x2776);
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 1;
  if (pvVar3 != (void *)0x0) {
    piVar9 = FUN_005fbd20(pvVar3,pHVar2,0x277c,2);
  }
  local_4 = 0xffffffff;
  if (piVar9 != (int *)0x0) {
    pvVar3 = (void *)FUN_00618b70(0x114);
    local_4 = 2;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      iVar1 = *(int *)((int)param_1 + 0x30);
      iVar15 = 1;
      iVar14 = 0;
      puVar5 = (uint *)FUN_006073d0(param_1);
      uVar13 = 0x277d;
      uVar12 = 0x277c;
      uVar11 = 1;
      pvVar10 = param_1;
      iVar6 = FUN_005fc0f0(piVar9);
      iVar7 = FUN_005fc0e0(piVar9);
      uVar4 = 3;
      iVar8 = FUN_005fc0e0(piVar9);
      pvVar3 = FUN_00602150(pvVar3,pHVar2,(iVar1 - iVar8) + -3,uVar4,iVar7,iVar6,pvVar10,uVar11,
                            uVar12,uVar13,puVar5,iVar14,iVar15);
    }
    local_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x144) = pvVar3;
    if (pvVar3 != (void *)0x0) {
      FUN_005ffce0(pvVar3,0);
      FUN_005fbfa0(piVar9);
      FUN_00618b60((undefined *)piVar9);
    }
  }
switchD_00452389_caseD_13:
  ExceptionList = local_c;
  return;
}

