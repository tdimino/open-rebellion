
void * __thiscall
FUN_0060d590(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            undefined4 param_6,undefined4 param_7,uint *param_8)

{
  HGDIOBJ pvVar1;
  void *this_00;
  uint *puVar2;
  undefined4 uVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  uint uVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00656bb3;
  pvStack_c = ExceptionList;
  uVar7 = 0;
  uVar6 = 0;
  uVar5 = 0;
  iVar4 = 0;
  uVar3 = 0;
  ExceptionList = &pvStack_c;
  pvVar1 = GetStockObject(0);
  FUN_005ff910(this,param_1,s_CoolTabControl_006ac930,0x56000000,param_2,param_3,param_4,param_5,
               param_7,(char *)0x0,pvVar1,uVar3,iVar4,uVar5,uVar6,uVar7);
  iStack_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066e230;
  *(undefined4 *)((int)this + 0x24) = param_6;
  *(undefined4 *)((int)this + 0x94) = 0;
  this_00 = (void *)FUN_00618b70(0x28);
  iStack_4._0_1_ = 1;
  if (this_00 == (void *)0x0) {
    puVar2 = (uint *)0x0;
  }
  else {
    puVar2 = FUN_005fbda0(this_00,param_4,param_5,param_8);
  }
  iStack_4 = (uint)iStack_4._1_3_ << 8;
  *(uint **)((int)this + 0x98) = puVar2;
  FUN_005fcc30(param_8,puVar2,0,0,param_2,param_3,(undefined4 *)0x0,0);
  ExceptionList = pvStack_c;
  return this;
}

