
void * __thiscall
FUN_004ac120(void *this,undefined4 param_1,int param_2,uint param_3,int param_4,int param_5,
            int param_6,undefined4 param_7,uint *param_8)

{
  HGDIOBJ pvVar1;
  void *pvVar2;
  uint *puVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063852a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00606380(this,param_1,param_2,param_3,param_4,param_5,param_6,0,0,0,param_7,0x52000000,0,0,0);
  local_4 = 0;
  FUN_004acb50((undefined4 *)((int)this + 0x114));
  local_4._0_1_ = 1;
  FUN_004f4340((undefined4 *)((int)this + 0x11c));
  local_4._0_1_ = 2;
  *(undefined ***)this = &PTR_FUN_0065bf80;
  FUN_005fffb0(this,s_CustomDialogBox_006a8828);
  *(undefined4 *)((int)this + 0x50) = 0x56000000;
  pvVar1 = GetStockObject(5);
  *(HGDIOBJ *)((int)this + 0x5c) = pvVar1;
  pvVar2 = (void *)FUN_00618b70(0x34);
  local_4._0_1_ = 3;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00601880(pvVar2,(char *)0x0,0,0,0,0,0,0);
  }
  local_4._0_1_ = 2;
  *(void **)((int)this + 0x124) = pvVar2;
  FUN_00601b30(pvVar2,2,2);
  **(int **)((int)this + 0x124) = param_4;
  *(undefined4 *)(*(int *)((int)this + 0x124) + 0x2c) = 3;
  FUN_00403e90(*(void **)((int)this + 0x124),0x25);
  SetRect((LPRECT)((int)this + 0x128),2,2,*(int *)((int)this + 0x38) + -3,0x10);
  if (param_8 != (uint *)0x0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (pvVar2 == (void *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = (uint *)FUN_005fbda0(pvVar2,param_4,param_5,param_8);
    }
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_005fcc30(param_8,puVar3,0,0,param_2,param_3,(undefined4 *)0x0,0);
    FUN_006075b0(this,(int *)puVar3,0);
  }
  *(undefined4 *)((int)this + 0x138) = 0;
  *(undefined4 *)((int)this + 0x13c) = 0;
  ExceptionList = pvStack_c;
  return this;
}

