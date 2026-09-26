
void * __thiscall
FUN_004acec0(void *this,undefined4 param_1,undefined4 param_2,int param_3,int param_4,int param_5,
            COLORREF param_6,COLORREF param_7,uint *param_8,undefined4 param_9,undefined4 param_10)

{
  void *this_00;
  int iVar1;
  int iVar2;
  uint *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638733;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0060e150(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,0x54000000);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065c058;
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xb8) = 0;
  *(undefined4 *)((int)this + 0xc0) = param_9;
  *(undefined4 *)((int)this + 0xc4) = param_10;
  if (param_8 != (uint *)0x0) {
    this_00 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 1;
    if (this_00 == (void *)0x0) {
      puVar3 = (uint *)0x0;
    }
    else {
      puVar3 = param_8;
      iVar1 = FUN_005fc0f0((int *)param_8);
      iVar2 = FUN_005fc0e0((int *)param_8);
      puVar3 = FUN_005fbda0(this_00,iVar2,iVar1,puVar3);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_004ad040(this,puVar3);
  }
  ExceptionList = local_c;
  return this;
}

