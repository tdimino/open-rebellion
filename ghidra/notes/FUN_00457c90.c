
void __thiscall FUN_00457c90(void *this,undefined4 *param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  void *this_00;
  undefined1 uVar3;
  int *piVar4;
  void *pvVar5;
  undefined4 *this_01;
  char *pcVar6;
  uint *puVar7;
  undefined4 *puVar8;
  undefined2 local_20;
  undefined2 local_1e;
  undefined4 local_1c;
  int local_18;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  this_00 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006304cc;
  local_c = ExceptionList;
  local_18 = *(int *)((int)param_2 + 0x58);
  iVar1 = (int)param_1 * 0x38;
  ExceptionList = &local_c;
  FUN_0040d760(param_2,&param_2);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  piVar4 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),(uint *)&param_2);
  local_14 = *(void **)((int)this + iVar1 + 400);
  puVar8 = *(undefined4 **)((int)this + iVar1 + 0x184);
  if (puVar8 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar8);
    FUN_00618b60((undefined *)puVar8);
    *(undefined4 *)((int)this + iVar1 + 0x184) = 0;
  }
  if ((local_18 == 0) || (piVar4 == (int *)0x0)) {
    FUN_00458080(this,(int)param_1,this_00);
    puVar8 = param_1;
    goto LAB_00457eb7;
  }
  uVar2 = *(uint *)(piVar4[0xb] + 0x30);
  local_1c = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 1;
  if (local_1c == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_005fbd20(local_1c,*(undefined4 *)((int)this + 0x168),uVar2 & 0xfff,10);
  }
  puVar8 = param_1;
  local_4._0_1_ = 0;
  uVar3 = (undefined1)local_4;
  local_4._0_1_ = 0;
  *(void **)((int)this + iVar1 + 0x184) = pvVar5;
  if (param_1 == (undefined4 *)0x1) {
    local_4._0_1_ = uVar3;
    local_10 = (void *)FUN_00618b70(0xc);
    local_4._0_1_ = 2;
    if (local_10 == (void *)0x0) {
LAB_00457e5c:
      this_01 = (undefined4 *)0x0;
    }
    else {
      local_1c = (void *)CONCAT22(DAT_0065d424,0x1841);
      this_01 = FUN_005f2fc0(local_10,&local_1c);
    }
  }
  else if (param_1 == (undefined4 *)&DAT_00000002) {
    local_4._0_1_ = uVar3;
    local_10 = (void *)FUN_00618b70(0xc);
    local_4._0_1_ = 3;
    if (local_10 == (void *)0x0) goto LAB_00457e5c;
    local_1e = DAT_0065d424;
    local_20 = 0x1840;
    this_01 = FUN_005f2fc0(local_10,&local_20);
  }
  else {
    this_01 = param_1;
    if (param_1 == (undefined4 *)0x3) {
      local_10 = (void *)FUN_00618b70(0xc);
      local_4._0_1_ = 4;
      if (local_10 == (void *)0x0) goto LAB_00457e5c;
      param_1 = (undefined4 *)CONCAT22(DAT_0065d424,0x1841);
      this_01 = FUN_005f2fc0(local_10,&param_1);
    }
  }
  local_4._0_1_ = 0;
  FUN_005f31a0(this_01,local_18);
  pcVar6 = (char *)FUN_00583c40((int)this_01);
  FUN_00601aa0(local_14,pcVar6);
  if (this_01 != (undefined4 *)0x0) {
    (**(code **)*this_01)(1);
  }
  FUN_004f26d0((void *)((int)this + iVar1 + 0x178),&param_2);
LAB_00457eb7:
  puVar7 = FUN_004591b0(this_00,&param_1);
  local_4._0_1_ = 5;
  FUN_00457f30(this,(int)puVar8,this_00,puVar7);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  FUN_00457b40(this,(int)puVar8,this_00,0);
  FUN_00458040(this,(int)puVar8,this_00,*(undefined4 *)((int)this_00 + 0x5c),
               *(int *)((int)this_00 + 0x68));
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

