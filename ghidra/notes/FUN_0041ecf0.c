
void * __thiscall
FUN_0041ecf0(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            undefined4 param_6,uint *param_7,int param_8)

{
  HCURSOR pHVar1;
  HGDIOBJ pvVar2;
  void *this_00;
  uint *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062b849;
  pvStack_c = ExceptionList;
  uVar7 = 0;
  uVar6 = 0;
  uVar5 = 0;
  ExceptionList = &pvStack_c;
  pHVar1 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
  uVar4 = 0;
  pvVar2 = GetStockObject(2);
  FUN_005ff910(this,param_1,s_TextScrollField_006a8598,0x56000000,param_2,param_3,param_4,param_5,
               param_6,(char *)0x0,pvVar2,uVar4,(int)pHVar1,uVar5,uVar6,uVar7);
  uStack_4 = 0;
  FUN_0060a420((undefined4 *)((int)this + 0x94));
  uStack_4._0_1_ = 1;
  FUN_005f2f50((undefined4 *)((int)this + 0xa0));
  uStack_4._0_1_ = 2;
  FUN_005f3d80((undefined4 *)((int)this + 0x100));
  uStack_4._0_1_ = 3;
  FUN_005f3d80((undefined4 *)((int)this + 0x10c));
  uStack_4._0_1_ = 4;
  FUN_005f5830((undefined4 *)((int)this + 0x160));
  uStack_4._0_1_ = 5;
  *(undefined ***)this = &PTR_FUN_00658ad0;
  *(undefined4 *)((int)this + 0x94) = &PTR_LAB_00658aa0;
  this_00 = (void *)FUN_00618b70(0x28);
  uStack_4._0_1_ = 6;
  if (this_00 == (void *)0x0) {
    puVar3 = (uint *)0x0;
  }
  else {
    puVar3 = (uint *)FUN_005fbda0(this_00,param_4,param_5,param_7);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,5);
  *(uint **)((int)this + 0xac) = puVar3;
  FUN_005fcc30(param_7,puVar3,0,0,param_2,param_3,(undefined4 *)0x0,0);
  *(undefined4 *)((int)this + 0xb0) = 0;
  *(undefined4 *)((int)this + 0xb4) = 0x2ffffff;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x138) = 0;
  FUN_0041fb50(this,param_8);
  *(undefined4 *)((int)this + 0x140) = 0;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x13c) = 0;
  *(undefined4 *)((int)this + 300) = 0;
  *(int *)((int)this + 0x130) = param_4;
  *(undefined4 *)((int)this + 0x11c) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x120) = 0;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined2 *)((int)this + 0x134) = 7;
  *(undefined2 *)((int)this + 0x136) = 0x299a;
  uVar4 = FUN_006002b0(4);
  *(undefined4 *)((int)this + 0xb0) = uVar4;
  ExceptionList = pvStack_c;
  return this;
}

