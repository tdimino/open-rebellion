
void __fastcall FUN_00437f80(void *param_1)

{
  bool bVar1;
  void *pvVar2;
  void *this;
  int iVar3;
  int *piVar4;
  uint *puVar5;
  uint uVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined4 uVar11;
  undefined4 *puVar12;
  undefined4 uStack_28;
  void *local_24;
  void *local_20;
  void *pvStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d6ed;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar2 = (void *)FUN_006037f0(7);
  local_20 = pvVar2;
  this = (void *)FUN_00618b70(0xf8);
  local_4 = 0;
  local_24 = this;
  if (this == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    uVar11 = 2;
    uVar10 = 0x299a;
    uVar9 = 7;
    uVar8 = 0;
    uVar7 = 0;
    iVar3 = FUN_006073d0(param_1);
    pvVar2 = FUN_0060bed0(this,pvVar2,(int)param_1,6,0x16,0xc3,0x3f,2,(int *)((int)param_1 + 0x118),
                          iVar3,uVar7,uVar8,uVar9,uVar10,uVar11);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x114) = pvVar2;
  if (pvVar2 == (void *)0x0) goto LAB_00438267;
  FUN_005ffce0(pvVar2,0);
  *(undefined4 *)(*(int *)((int)param_1 + 0x114) + 0x98) = 0x101;
  (**(code **)(**(int **)((int)param_1 + 0x114) + 0x18))(10);
  *(undefined4 *)(*(int *)((int)param_1 + 0x114) + 0xb4) = 0;
  local_20 = (void *)FUN_00618b70(0x28);
  puStack_8 = (undefined1 *)0x1;
  if (local_20 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_005fbd20(local_20,local_24,0x2966,2);
  }
  puStack_8 = (undefined1 *)0xffffffff;
  *(void **)(*(int *)((int)param_1 + 0x114) + 0xf4) = pvVar2;
  *(undefined4 *)(*(int *)((int)param_1 + 0x114) + 0xa4) = 0x46;
  *(undefined4 *)(*(int *)((int)param_1 + 0x114) + 0x94) = 0x2ffffff;
  FUN_0042dbe0(&uStack_28);
  puStack_8 = (undefined1 *)0x2;
  iVar3 = (**(code **)(*(int *)((int)param_1 + 0x118) + 8))();
  if (iVar3 != 0) {
    pvStack_1c = (void *)0x20;
    uStack_18 = 0x30;
    uVar6 = **(uint **)(iVar3 + 0x54) >> 0x18;
    if (((uVar6 < 0x20) || (0x2f < uVar6)) || (uStack_28._3_1_ = '\x01', DAT_006b289c == local_24))
    {
      uStack_28._3_1_ = '\0';
    }
    FUN_00619730();
    if (uStack_28._3_1_ == '\0') {
      pvStack_1c = (void *)0x14;
      uStack_18 = 0x20;
      if ((uVar6 < 0x14) || (0x1f < uVar6)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        if (DAT_006b28a4 != local_24) {
          puVar12 = &DAT_006b28a4;
          goto LAB_004381ed;
        }
      }
      else {
        uStack_14 = 0x10;
        uStack_10 = 0x14;
        local_4 = CONCAT31(local_4._1_3_,3);
        if ((uVar6 < 0x10) || (0x13 < uVar6)) {
          pvStack_1c = (void *)0x3c;
          uStack_18 = 0x40;
          if ((uVar6 < 0x3c) || (0x3f < uVar6)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          uStack_28._3_1_ = '\0';
          if (bVar1) goto LAB_004381c0;
        }
        else {
LAB_004381c0:
          uStack_28._3_1_ = '\x01';
        }
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_00619730();
        if ((uStack_28._3_1_ != '\0') && (DAT_006b28a0 != local_24)) {
          puVar12 = &DAT_006b28a0;
          goto LAB_004381ed;
        }
      }
    }
    else {
      puVar12 = &DAT_006b289c;
LAB_004381ed:
      FUN_004f26d0((void *)((int)param_1 + 0x150),puVar12);
    }
    if (*(void **)((int)param_1 + 0x150) != local_24) {
      iVar3 = FUN_0051cab0((uint *)((int)param_1 + 0x150));
      uVar11 = *(undefined4 *)(iVar3 + 0x30);
      for (piVar4 = (int *)(**(code **)(*(int *)((int)param_1 + 0x118) + 8))(); piVar4 != (int *)0x0
          ; piVar4 = (int *)(**(code **)(*piVar4 + 0xc))()) {
        iVar3 = FUN_0051cab0((uint *)piVar4[0x15]);
        if ((short)uVar11 == *(short *)(iVar3 + 0x30)) {
          FUN_0060c970(*(void **)((int)param_1 + 0x114),piVar4[3]);
          break;
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_00438267:
  pvStack_1c = (void *)FUN_00618b70(0x34);
  local_4 = 4;
  if (pvStack_1c == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00601880(pvStack_1c,(char *)&lpString_006a7f40,0x24,0x6e,0x40,0x17,0x25,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x134) = pvVar2;
  if (pvVar2 != (void *)0x0) {
    FUN_00601c60(pvVar2,4);
    FUN_00601c90(*(void **)((int)param_1 + 0x134),0x2ffffff,(HWND)0x0);
  }
  pvStack_1c = (void *)FUN_00618b70(0x34);
  local_4 = 5;
  if (pvStack_1c == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00601880(pvStack_1c,(char *)&lpString_006a7f40,0x8a,0x6e,0x40,0x17,0x25,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x138) = pvVar2;
  if (pvVar2 != (void *)0x0) {
    FUN_00601c60(pvVar2,4);
    FUN_00601c90(*(void **)((int)param_1 + 0x138),0x2ffffff,(HWND)0x0);
  }
  FUN_00438dd0(param_1,0,0,0);
  pvStack_1c = (void *)FUN_00618b70(0x34);
  local_4 = 6;
  if (pvStack_1c == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00601880(pvStack_1c,(char *)&lpString_006a7f40,0x8c,0x91,0x3c,0xf,0x26,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 300) = pvVar2;
  if (pvVar2 != (void *)0x0) {
    FUN_00601c60(pvVar2,10);
    FUN_00601c90(*(void **)((int)param_1 + 300),0x2ffffff,(HWND)0x0);
  }
  pvStack_1c = (void *)FUN_00618b70(0x34);
  local_4 = 7;
  if (pvStack_1c == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00601880(pvStack_1c,(char *)&lpString_006a7f40,0x8c,0xa5,0x3c,0xf,0x26,3);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x130) = pvVar2;
  if (pvVar2 != (void *)0x0) {
    FUN_00601c60(pvVar2,10);
    FUN_00601c90(*(void **)((int)param_1 + 0x130),0x2ffffff,(HWND)0x0);
  }
  FUN_00438f30(param_1,0,0,0);
  pvVar2 = (void *)FUN_00618b70(0xf4);
  local_4 = 8;
  pvStack_1c = pvVar2;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    uVar6 = 0;
    puVar5 = (uint *)FUN_006073d0(param_1);
    pvVar2 = FUN_00604cf0(pvVar2,local_20,0x8d,0xc4,0x2d,0x11,(int)param_1,puVar5,uVar6);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x128) = pvVar2;
  if (pvVar2 != (void *)0x0) {
    *(undefined4 *)((int)pvVar2 + 0x24) = 0x6b;
    (**(code **)(**(int **)((int)param_1 + 0x128) + 0x18))(4);
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0xe0) = 0;
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0xe4) = 0x2ffffff;
    *(undefined4 *)(*(int *)((int)param_1 + 0x128) + 0xec) = 0x200ff00;
    FUN_005ffce0(*(void **)((int)param_1 + 0x128),0);
    FUN_006050c0(*(void **)((int)param_1 + 0x128),0x19);
    FUN_00438c90((int)param_1);
    FUN_006009f0(param_1,*(void **)((int)param_1 + 0x128),0x190c);
    SetFocus(*(HWND *)(*(int *)((int)param_1 + 0x128) + 0x18));
  }
  FUN_00439160(param_1);
  ExceptionList = pvStack_c;
  return;
}

