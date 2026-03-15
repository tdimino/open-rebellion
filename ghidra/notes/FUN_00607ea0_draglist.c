
void * __thiscall
FUN_00607ea0(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            undefined4 param_6,uint *param_7,undefined4 param_8,undefined4 param_9,
            undefined4 param_10)

{
  HGDIOBJ pvVar1;
  void *pvVar2;
  uint *puVar3;
  int in_stack_0000003c;
  int in_stack_00000040;
  HCURSOR in_stack_00000044;
  undefined4 uVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  uint uVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656710;
  pvStack_c = ExceptionList;
  uVar8 = 0;
  uVar7 = 0;
  uVar6 = 0;
  iVar5 = 0;
  uVar4 = 0;
  ExceptionList = &pvStack_c;
  pvVar1 = GetStockObject(4);
  FUN_005ff910(this,param_1,PTR_s_CoolDragList_0066e0b8,0x56010000,param_2,param_3,param_4,param_5,
               param_6,(char *)0x0,pvVar1,uVar4,iVar5,uVar6,uVar7,uVar8);
  uStack_4 = 0;
  FUN_0060a420((undefined4 *)((int)this + 0x94));
  uStack_4._0_1_ = 1;
  FUN_005f4950((void *)((int)this + 0xf4),0);
  uStack_4._0_1_ = 2;
  *(undefined ***)this = &PTR_FUN_0066e0f0;
  *(undefined4 *)((int)this + 0x94) = &PTR_LAB_0066e0c0;
  pvVar2 = (void *)FUN_00618b70(0x10);
  uStack_4._0_1_ = 3;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_0060a790(pvVar2,1);
  }
  *(void **)((int)this + 0xa0) = pvVar2;
  *(undefined4 *)((int)this + 0x24) = param_8;
  uStack_4._0_1_ = 2;
  *(undefined4 *)((int)this + 0xb8) = 0;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(int *)((int)this + 0xf0) = in_stack_00000040;
  *(undefined4 *)((int)this + 0xcc) = param_9;
  *(undefined4 *)((int)this + 0xd0) = param_10;
  if (in_stack_00000040 == 0) {
    *(undefined4 *)((int)this + 0xb8) = param_10;
  }
  else {
    *(undefined4 *)((int)this + 0xbc) = param_9;
  }
  *(undefined4 *)((int)this + 0xd8) = 0x787878;
  *(undefined4 *)((int)this + 0xdc) = 0xffffff;
  *(undefined1 *)((int)this + 0xd4) = 0;
  pvVar2 = (void *)FUN_00618b70(0x28);
  uStack_4._0_1_ = 4;
  if (pvVar2 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_005fbda0(pvVar2,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),
                                  param_7);
  }
  uStack_4._0_1_ = 2;
  *(uint **)((int)this + 0xa8) = puVar3;
  FUN_005fcc30(param_7,puVar3,0,0,param_2,param_3,(undefined4 *)0x0,0);
  pvVar2 = (void *)FUN_00618b70(0x28);
  uStack_4._0_1_ = 5;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_005fbda0(pvVar2,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),param_7);
  }
  uStack_4._0_1_ = 2;
  *(void **)((int)this + 0xa4) = pvVar2;
  pvVar2 = (void *)FUN_00618b70(0x28);
  uStack_4._0_1_ = 6;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_005fbda0(pvVar2,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),param_7);
  }
  *(void **)((int)this + 0xac) = pvVar2;
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  *(undefined4 *)((int)this + 0x110) = 0;
  *(undefined4 *)((int)this + 0xc4) = 0;
  *(undefined4 *)((int)this + 200) = 0;
  *(int *)((int)this + 0xc0) = in_stack_0000003c;
  if (in_stack_0000003c == 0) {
    *(undefined4 *)((int)this + 0xc0) = 10;
  }
  *(HCURSOR *)((int)this + 0xb4) = in_stack_00000044;
  iVar5 = DAT_006be5b4;
  if (in_stack_00000044 == (HCURSOR)0x0) {
    *(int *)((int)this + 0xb0) = DAT_006be5b4;
    if (iVar5 != 0) goto LAB_006080db;
    in_stack_00000044 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
  }
  *(HCURSOR *)((int)this + 0xb0) = in_stack_00000044;
LAB_006080db:
  *(undefined4 *)((int)this + 0xe4) = 0x29;
  *(undefined2 *)((int)this + 0xfc) = 0xfc18;
  *(undefined2 *)((int)this + 0xfe) = 0xfc18;
  *(undefined4 *)((int)this + 0xe8) = 1;
  *(undefined4 *)((int)this + 0xec) = 0;
  *(undefined4 *)((int)this + 0x104) = 0;
  uVar4 = *(undefined4 *)(*(int *)((int)this + 0x20) + 0x18);
  *(undefined4 *)((int)this + 0x100) = 0;
  *(undefined4 *)((int)this + 0x114) = uVar4;
  ExceptionList = pvStack_c;
  return this;
}

