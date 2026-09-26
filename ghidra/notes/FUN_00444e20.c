
void __thiscall FUN_00444e20(void *this,int *param_1,void *param_2)

{
  void *pvVar1;
  char *pcVar2;
  undefined *puVar3;
  uint uVar4;
  int iVar5;
  int *piVar6;
  int iVar7;
  uint uVar8;
  int local_48;
  undefined4 local_44;
  undefined4 local_40;
  int local_38;
  int local_34;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ee78;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_30);
  local_4 = 0;
  local_44 = CONCAT22(DAT_0065d424,0x8663);
  FUN_005f3010(local_30,&local_44);
  pcVar2 = (char *)FUN_00583c40((int)local_30);
  FUN_00601aa0((void *)((int)this + 0x130),pcVar2);
  pvVar1 = param_2;
  piVar6 = param_1;
  iVar7 = 1;
  FUN_0044a210((int)this);
  local_38 = 1;
  local_34 = -1;
  local_4._0_1_ = 1;
  local_48 = 0;
  FUN_004fd2b0(*(int **)((int)this + 300),&local_38,&local_48);
  if ((local_34 == -1) && (local_48 != 0)) {
    param_2 = (void *)CONCAT22(DAT_0065d424,0x3814);
    FUN_005f2fc0(local_18,&param_2);
    local_4._0_1_ = 2;
    FUN_005f3c20(local_30);
    param_2 = (void *)CONCAT22(param_2._2_2_,0x8766);
    FUN_005f2fc0(local_24,&param_2);
    local_4._0_1_ = 3;
    pcVar2 = (char *)FUN_00583c40((int)local_24);
    FUN_005f35b0(&local_44,pcVar2);
    local_4._0_1_ = 4;
    FUN_005f30d0(local_30,(int)&local_44);
    local_4._0_1_ = 3;
    FUN_005f2ff0(&local_44);
    iVar7 = 2;
    FUN_0044b1d0(2,(int)local_30,piVar6);
    FUN_005f3c20(local_30);
    pcVar2 = (char *)FUN_00583c40((int)local_18);
    FUN_005f35b0(&local_44,pcVar2);
    local_4._0_1_ = 5;
    FUN_005f30d0(local_30,(int)&local_44);
    FUN_005f31a0(local_30,local_48);
    local_4._0_1_ = 3;
    FUN_005f2ff0(&local_44);
    FUN_0044b1d0(2,(int)local_30,pvVar1);
    FUN_005f3c20(local_30);
    local_4._0_1_ = 2;
    FUN_005f2ff0(local_24);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_18);
  }
  param_2 = (void *)CONCAT22(DAT_0065d424,0x8664);
  FUN_005f3010(local_30,&param_2);
  FUN_0044b1d0(iVar7 + 1,(int)local_30,piVar6);
  FUN_005f3c20(local_30);
  puVar3 = FUN_004f62d0(*(int *)(*(int *)((int)this + 300) + 0x1c));
  pcVar2 = (char *)FUN_00583c40((int)puVar3);
  FUN_005f35b0(local_18,pcVar2);
  local_4._0_1_ = 6;
  FUN_005f30d0(local_30,(int)local_18);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_005f2ff0(local_18);
  FUN_0044b1d0(iVar7 + 1,(int)local_30,pvVar1);
  FUN_0044a470(this,(void *)(iVar7 + 2),piVar6,pvVar1);
  uVar8 = iVar7 + 3;
  FUN_0044a620(this,uVar8,piVar6,pvVar1);
  local_44 = 0x22;
  uVar4 = *(uint *)((int)this + 0x124) >> 0x18;
  local_40 = 0x28;
  if ((uVar4 < 0x22) || (param_2 = (void *)0x1, 0x27 < uVar4)) {
    param_2 = (void *)0x0;
  }
  FUN_00619730();
  if (param_2 != (void *)0x0) {
    param_2 = (void *)CONCAT22(DAT_0065d424,0x8659);
    FUN_005f3010(local_30,&param_2);
    FUN_0044b1d0(iVar7 + 4,(int)local_30,piVar6);
    FUN_005f3c20(local_30);
    piVar6 = *(int **)((int)this + 300);
    param_2 = (void *)(**(code **)(*piVar6 + 0x1d4))();
    iVar5 = (**(code **)(*piVar6 + 0x1cc))();
    FUN_005f31f0(local_30,iVar5 - (int)param_2);
    FUN_0044b1d0(iVar7 + 4,(int)local_30,pvVar1);
    param_2 = (void *)CONCAT22(DAT_0065d424,0x8743);
    FUN_005f3010(local_30,&param_2);
    piVar6 = param_1;
    uVar8 = iVar7 + 5;
    FUN_0044b1d0(uVar8,(int)local_30,param_1);
    FUN_005f3c20(local_30);
    param_1 = *(int **)((int)this + 300);
    param_2 = (void *)(**(code **)(*param_1 + 0x1d0))();
    iVar7 = (**(code **)(*param_1 + 0x1d8))();
    FUN_005f31f0(local_30,(int)param_2 - iVar7);
    FUN_0044b1d0(uVar8,(int)local_30,pvVar1);
  }
  param_1 = (int *)CONCAT22(DAT_0065d424,0x8669);
  FUN_005f3010(local_30,&param_1);
  FUN_0044b1d0(uVar8 + 1,(int)local_30,piVar6);
  FUN_005f3c20(local_30);
  piVar6 = *(int **)((int)this + 300);
  param_2 = (void *)(**(code **)(*piVar6 + 0x1c4))();
  iVar7 = (**(code **)(*piVar6 + 0x1c0))();
  FUN_005f31f0(local_30,iVar7 - (int)param_2);
  FUN_0044b1d0(uVar8 + 1,(int)local_30,pvVar1);
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_30);
  ExceptionList = pvStack_c;
  return;
}

