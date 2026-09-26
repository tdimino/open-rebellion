
void __thiscall FUN_00445780(void *this,void *param_1,void *param_2)

{
  int *piVar1;
  void *pvVar2;
  void *pvVar3;
  char *pcVar4;
  undefined4 uVar5;
  int iVar6;
  int local_50;
  undefined2 local_4c;
  undefined2 local_4a;
  int local_48 [6];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ef08;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_48 + 3);
  local_4a = DAT_0065d424;
  local_4 = 0;
  local_4c = 0x8670;
  FUN_005f3010(local_48 + 3,&local_4c);
  pcVar4 = (char *)FUN_00583c40((int)(local_48 + 3));
  FUN_00601aa0((void *)((int)this + 0x130),pcVar4);
  pvVar3 = param_2;
  pvVar2 = param_1;
  piVar1 = *(int **)((int)this + 300);
  FUN_0044a210((int)this);
  FUN_0044a2e0(this,2,pvVar2,pvVar3);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8666);
  FUN_005f3010(local_48 + 3,&param_1);
  iVar6 = 3;
  FUN_0044b1d0(3,(int)(local_48 + 3),pvVar2);
  FUN_005f3c20(local_48 + 3);
  FUN_004ece30(local_48);
  local_4._0_1_ = 1;
  if ((piVar1[0x14] & 0x10U) == 0) {
    if (piVar1[0x1a] == local_48[0]) {
      if ((piVar1[0x14] & 4U) == 0) {
        param_1 = (void *)CONCAT22(DAT_0065d424,0x8757);
      }
      else {
        param_1 = (void *)CONCAT22(DAT_0065d424,0x8748);
      }
    }
    else {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8745);
    }
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8744);
  }
  FUN_005f3010(local_48 + 3,&param_1);
  FUN_0044b1d0(3,(int)(local_48 + 3),param_2);
  local_48[1] = 1;
  local_48[2] = -1;
  local_4._0_1_ = 2;
  local_50 = 0;
  FUN_004fd2b0(*(int **)((int)this + 300),local_48 + 1,&local_50);
  if ((local_48[2] == -1) && (local_50 != 0)) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x3814);
    FUN_005f2fc0(local_18,&param_1);
    local_4._0_1_ = 3;
    FUN_005f3c20(local_48 + 3);
    param_1 = (void *)CONCAT22(param_1._2_2_,0x8766);
    FUN_005f2fc0(local_24,&param_1);
    local_4._0_1_ = 4;
    pcVar4 = (char *)FUN_00583c40((int)local_24);
    FUN_005f35b0(local_30,pcVar4);
    local_4._0_1_ = 5;
    FUN_005f30d0(local_48 + 3,(int)local_30);
    local_4._0_1_ = 4;
    FUN_005f2ff0(local_30);
    iVar6 = 4;
    FUN_0044b1d0(4,(int)(local_48 + 3),pvVar2);
    FUN_005f3c20(local_48 + 3);
    pcVar4 = (char *)FUN_00583c40((int)local_18);
    FUN_005f35b0(local_30,pcVar4);
    local_4._0_1_ = 6;
    FUN_005f30d0(local_48 + 3,(int)local_30);
    FUN_005f31a0(local_48 + 3,local_50);
    local_4._0_1_ = 4;
    FUN_005f2ff0(local_30);
    FUN_0044b1d0(4,(int)(local_48 + 3),param_2);
    FUN_005f3c20(local_48 + 3);
    local_4._0_1_ = 3;
    FUN_005f2ff0(local_24);
    local_4._0_1_ = 2;
    FUN_005f2ff0(local_18);
  }
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8651);
  FUN_005f3010(local_48 + 3,&param_1);
  FUN_0044b1d0(iVar6 + 1,(int)(local_48 + 3),pvVar2);
  FUN_005f3c20(local_48 + 3);
  uVar5 = FUN_004f2990(*(int *)((int)this + 300));
  FUN_005f31f0(local_48 + 3,uVar5);
  FUN_0044b1d0(iVar6 + 1,(int)(local_48 + 3),param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8672);
  FUN_005f3010(local_48 + 3,&param_1);
  FUN_0044b1d0(iVar6 + 2,(int)(local_48 + 3),pvVar2);
  FUN_005f3c20(local_48 + 3);
  uVar5 = (**(code **)(*piVar1 + 0x1dc))();
  FUN_005f31f0(local_48 + 3,uVar5);
  FUN_0044b1d0(iVar6 + 2,(int)(local_48 + 3),param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8673);
  FUN_005f3010(local_48 + 3,&param_1);
  FUN_0044b1d0(iVar6 + 3,(int)(local_48 + 3),pvVar2);
  FUN_005f3c20(local_48 + 3);
  uVar5 = (**(code **)(*piVar1 + 0x1e0))();
  FUN_005f31f0(local_48 + 3,uVar5);
  FUN_0044b1d0(iVar6 + 3,(int)(local_48 + 3),param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8674);
  FUN_005f3010(local_48 + 3,&param_1);
  FUN_0044b1d0(iVar6 + 4,(int)(local_48 + 3),pvVar2);
  FUN_005f3c20(local_48 + 3);
  uVar5 = (**(code **)(*piVar1 + 0x1f0))();
  FUN_005f31f0(local_48 + 3,uVar5);
  FUN_0044b1d0(iVar6 + 4,(int)(local_48 + 3),param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8675);
  FUN_005f3010(local_48 + 3,&param_1);
  FUN_0044b1d0(iVar6 + 5,(int)(local_48 + 3),pvVar2);
  FUN_005f3c20(local_48 + 3);
  uVar5 = (**(code **)(*piVar1 + 500))();
  FUN_005f31f0(local_48 + 3,uVar5);
  FUN_0044b1d0(iVar6 + 5,(int)(local_48 + 3),param_2);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_48 + 3);
  ExceptionList = pvStack_c;
  return;
}

