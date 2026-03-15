
void * __thiscall
FUN_0045d400(void *this,undefined4 param_1,undefined4 param_2,undefined4 *param_3,uint param_4,
            int *param_5,uint param_6,int *param_7,undefined4 *param_8,undefined4 *param_9)

{
  uint uVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  uint *puVar5;
  undefined4 uVar6;
  LPBYTE pBVar7;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  uVar1 = param_6;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630cab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,param_2,param_3,param_4,(int)param_5,param_6,0,7,0,param_7,0,1,0,0);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_00659fa8;
  FUN_004f26d0(&param_7,param_8);
  local_4._0_1_ = 1;
  FUN_004f26d0(&param_6,param_9);
  local_4._0_1_ = 2;
  FUN_004ece30(&param_5);
  local_4._0_1_ = 3;
  FUN_0042dbe0(&param_4);
  local_4 = CONCAT31(local_4._1_3_,4);
  *(undefined4 *)((int)this + 0x50) = 0x56000000;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x128) = 0;
  *(undefined4 *)((int)this + 300) = 0;
  *(undefined4 *)((int)this + 0x120) = 0;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x13c) = 0;
  *(undefined4 *)((int)this + 0x140) = 0;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x134) = 0;
  *(uint *)((int)this + 0x160) = uVar1;
  *(undefined4 *)((int)this + 0x114) = 1;
  piVar2 = param_7;
  if ((param_7 != param_5) &&
     (piVar2 = FUN_004f2d10(*(int *)(uVar1 + 0x9c),(uint *)&param_7), piVar2 == (int *)0x0)) {
    FUN_004f26d0(&param_7,&param_5);
  }
  piVar3 = FUN_004f26e0((int *)&param_6);
  if ((((piVar3 != (int *)0x0) && (param_6 != param_4)) && (param_7 == param_5)) &&
     (param_3 = (undefined4 *)FUN_0051cab0(&param_6), param_3 == (undefined4 *)0x0)) {
    FUN_004f26d0(&param_6,&param_4);
  }
  if (param_7 == param_5) {
    if (param_6 == param_4) {
      *(undefined4 *)((int)this + 0x148) = 0;
      goto LAB_0045d69d;
    }
    uVar6 = FUN_0060a860(*(void **)(*(int *)((int)this + 0x160) + 0x474),
                         (param_3[0xc] & 0xfff) + 0x1000);
    *(undefined4 *)((int)this + 0x148) = uVar6;
  }
  else {
    iVar4 = FUN_0060a860(*(void **)(*(int *)((int)this + 0x160) + 0x474),
                         (*(uint *)(piVar2[0xb] + 0x30) & 0xfff) + 0x1000);
    *(int *)((int)this + 0x148) = iVar4;
    if (iVar4 == 0) {
      puVar5 = FUN_0045fd90(this,(uint *)&param_3,(uint *)&param_7);
      local_4._0_1_ = 5;
      piVar2 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x160) + 0x9c),puVar5);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00619730();
      if (piVar2 == (int *)0x0) {
        *(undefined4 *)((int)this + 0x148) = 0;
        *(undefined4 *)((int)this + 0x114) = 2;
      }
      else {
        uVar6 = FUN_0060a860(*(void **)(*(int *)((int)this + 0x160) + 0x474),
                             (*(uint *)(piVar2[0xb] + 0x30) & 0xfff) + 0x1000);
        *(undefined4 *)((int)this + 0x148) = uVar6;
        *(undefined4 *)((int)this + 0x114) = 2;
      }
      goto LAB_0045d69d;
    }
  }
  *(undefined4 *)((int)this + 0x114) = 2;
LAB_0045d69d:
  if (*(int *)((int)this + 0x148) == 0) {
    *(undefined4 *)((int)this + 0x114) = 1;
  }
  param_3 = (undefined4 *)FUN_00618b70(0xc);
  local_4._0_1_ = 6;
  if (param_3 == (undefined4 *)0x0) {
    uVar6 = 0;
  }
  else {
    uVar6 = FUN_005f2f50(param_3);
  }
  local_4 = CONCAT31(local_4._1_3_,4);
  *(undefined4 *)((int)this + 0x150) = uVar6;
  pBVar7 = FUN_00401440();
  FUN_00603810((char *)pBVar7,s_encytext_dll_006a8760,4);
  FUN_00603810((char *)pBVar7,s_encybmap_dll_006a8750,5);
  if (pBVar7 != (LPBYTE)0x0) {
    FUN_00618b60(pBVar7);
  }
  uVar6 = FUN_006037f0(4);
  *(undefined4 *)((int)this + 0x158) = uVar6;
  uVar6 = FUN_006037f0(5);
  *(undefined4 *)((int)this + 0x15c) = uVar6;
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = local_c;
  return this;
}

