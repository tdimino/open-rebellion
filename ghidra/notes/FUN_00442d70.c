
void * __thiscall
FUN_00442d70(void *this,undefined4 param_1,int param_2,undefined4 param_3,undefined4 *param_4)

{
  uint *this_00;
  int *this_01;
  undefined4 *puVar1;
  int iVar2;
  LPBYTE pBVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ebc6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,0,0,0x17b,0x110,param_2,2,7,0,param_3,0,1,0,0);
  this_00 = (uint *)((int)this + 0x124);
  local_4 = 0;
  FUN_004ece30(this_00);
  local_4._0_1_ = 1;
  FUN_0042dbe0((undefined4 *)((int)this + 0x128));
  local_4._0_1_ = 2;
  FUN_00601880((void *)((int)this + 0x130),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 3;
  FUN_00601880((void *)((int)this + 0x164),(char *)0x0,0,0,0,0,0,0);
  local_4._0_1_ = 4;
  FUN_00601880((void *)((int)this + 0x198),(char *)0x0,0,0,0,0,0,0);
  *(int *)((int)this + 0x114) = param_2;
  local_4._0_1_ = 5;
  *(undefined ***)this = &PTR_FUN_00659c18;
  *(undefined4 *)((int)this + 0x50) = 0x46000000;
  *(undefined4 *)((int)this + 0x120) = 0;
  FUN_004f26d0(this_00,param_4);
  this_01 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x114) + 0x9c),this_00);
  *(int **)((int)this + 300) = this_01;
  if (this_01 != (int *)0x0) {
    puVar1 = (undefined4 *)FUN_00402d80(this_01,&param_3);
    local_4._0_1_ = 6;
    FUN_004f26d0((undefined4 *)((int)this + 0x128),puVar1);
    local_4._0_1_ = 5;
    FUN_00619730();
  }
  iVar2 = FUN_005fefd0(10);
  *(int *)((int)this + 0x1d4) = iVar2;
  if (iVar2 == 0) {
    pBVar3 = FUN_00401440();
    iVar2 = FUN_005ff020((char *)pBVar3,s_gokres_dll_006a8708,10);
    *(int *)((int)this + 0x1d4) = iVar2;
    if (pBVar3 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar3);
    }
  }
  *(undefined4 *)((int)this + 0x11c) = 0;
  ExceptionList = local_c;
  return this;
}

