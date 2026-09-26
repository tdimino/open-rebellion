
void __fastcall FUN_0044f180(void *param_1)

{
  undefined4 uVar1;
  uint *puVar2;
  HDC hdc;
  void *pvVar3;
  int iVar4;
  int iVar5;
  uint *puVar6;
  char *pcVar7;
  HGDIOBJ pvVar8;
  undefined4 *this;
  undefined2 extraout_var;
  int iVar9;
  uint uVar10;
  undefined4 *puVar11;
  uint uVar12;
  undefined4 local_18;
  void *local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f9d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = FUN_006037f0(7);
  local_18 = uVar1;
  FUN_006037f0(2);
  iVar4 = *(int *)((int)param_1 + 0x114);
  FUN_00606980(param_1,*(int *)(iVar4 + 0xcc),*(int *)(iVar4 + 0xd0),*(int *)(iVar4 + 0xd4),
               *(int *)(iVar4 + 0xd8));
  if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
    local_14 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (local_14 != (void *)0x0) {
      puVar2 = FUN_005fbd20(local_14,uVar1,0x2b75,2);
      goto LAB_0044f24a;
    }
  }
  else {
    local_14 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (local_14 != (void *)0x0) {
      puVar2 = FUN_005fbd20(local_14,uVar1,0x2b76,2);
      goto LAB_0044f24a;
    }
  }
  puVar2 = (uint *)0x0;
LAB_0044f24a:
  local_4 = 0xffffffff;
  hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 2;
  local_14 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    puVar6 = (uint *)0x0;
  }
  else {
    puVar6 = puVar2;
    iVar4 = FUN_005fc0f0((int *)puVar2);
    iVar5 = FUN_005fc0e0((int *)puVar2);
    puVar6 = FUN_005fbda0(pvVar3,iVar5,iVar4,puVar6);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puVar2,puVar6,0,0,0,0,(undefined4 *)0x0,0);
  if (puVar2 != (uint *)0x0) {
    FUN_005fbfa0(puVar2);
    FUN_00618b60((undefined *)puVar2);
  }
  FUN_006075b0(param_1,(int *)puVar6,10);
  pvVar3 = (void *)FUN_00618b70(0x178);
  local_4 = 3;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    local_14 = *(void **)((int)param_1 + 0x1c);
    iVar4 = 0;
    puVar2 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_0041ecf0(pvVar3,local_14,0x18,0xf2,0x142,0x46,param_1,puVar2,iVar4);
  }
  local_4 = 0xffffffff;
  *(void **)((int)param_1 + 0x11c) = pvVar3;
  FUN_005ffce0(pvVar3,0);
  FUN_00420550(*(void **)((int)param_1 + 0x11c),4);
  pvVar3 = *(void **)((int)param_1 + 0x11c);
  FUN_005f3090((void *)((int)pvVar3 + 0xa0),*(int *)((int)param_1 + 0x118) + 0x50);
  *(undefined4 *)((int)pvVar3 + 0x13c) = 0;
  FUN_0041fc30(pvVar3);
  pvStack_10 = (void *)FUN_00618b70(0x34);
  local_4 = 4;
  if (pvStack_10 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
  }
  *(void **)((int)param_1 + 0x120) = pvVar3;
  *(undefined4 *)((int)pvVar3 + 0x2c) = 1;
  local_4 = 0xffffffff;
  FUN_00601b30(*(void **)((int)param_1 + 0x120),0x15,0xe);
  FUN_00601c60(*(void **)((int)param_1 + 0x120),4);
  FUN_00403e90(*(void **)((int)param_1 + 0x120),1);
  pcVar7 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x118) + 0x44);
  FUN_00601aa0(*(void **)((int)param_1 + 0x120),pcVar7);
  pvVar8 = (HGDIOBJ)FUN_005fc130((int)puVar6);
  pvVar8 = SelectObject(hdc,pvVar8);
  FUN_00601ce0(*(void **)((int)param_1 + 0x120),hdc);
  SelectObject(hdc,pvVar8);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 5;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
    uVar1 = local_18;
  }
  else {
    iVar5 = 0;
    iVar4 = 0;
    puVar2 = (uint *)FUN_006073d0(param_1);
    uVar1 = local_18;
    pvVar3 = FUN_00602150(pvVar3,local_18,0x163,0xf4,0x33,0x23,param_1,0x14,0x2aae,0x2aaf,puVar2,
                          iVar4,iVar5);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_005ffce0(pvVar3,0);
    local_18 = CONCAT22(DAT_0065d424,0x1300);
    FUN_00600a40(param_1,pvVar3,&local_18);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar3);
  }
  pvVar3 = (void *)FUN_00618b70(0x114);
  local_4 = 6;
  pvStack_10 = pvVar3;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    iVar5 = 0;
    iVar4 = 0;
    puVar2 = (uint *)FUN_006073d0(param_1);
    pvVar3 = FUN_00602150(pvVar3,uVar1,0x163,0x119,0x33,0x23,param_1,0x15,0x2ab1,0x2ab2,puVar2,iVar4
                          ,iVar5);
  }
  local_4 = 0xffffffff;
  if (pvVar3 != (void *)0x0) {
    FUN_005ffce0(pvVar3,0);
    local_18 = CONCAT22(DAT_0065d424,0x1301);
    FUN_00600a40(param_1,pvVar3,&local_18);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),(int)pvVar3);
  }
  pvStack_10 = (void *)FUN_00618b70(0x28);
  local_4 = 7;
  if (pvStack_10 == (void *)0x0) {
    this = (undefined4 *)0x0;
  }
  else {
    this = FUN_005fbd20(pvStack_10,uVar1,
                        CONCAT22(extraout_var,*(undefined2 *)(*(int *)((int)param_1 + 0x118) + 0x2e)
                                ),2);
  }
  local_4 = 0xffffffff;
  if (this != (undefined4 *)0x0) {
    uVar12 = 0;
    puVar11 = (undefined4 *)0x0;
    uVar10 = 0;
    iVar9 = 0;
    iVar5 = 0x1e;
    iVar4 = 0xc;
    puVar2 = (uint *)FUN_006073d0(param_1);
    FUN_005fcc30(this,puVar2,iVar4,iVar5,iVar9,uVar10,puVar11,uVar12);
    FUN_005fbfa0(this);
    FUN_00618b60((undefined *)this);
  }
  SetFocus(*(HWND *)((int)param_1 + 0x18));
  ExceptionList = pvStack_c;
  return;
}

