
bool __cdecl FUN_00559a60(int param_1,int param_2,undefined4 *param_3,int *param_4)

{
  undefined4 *puVar1;
  int *this;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *this_00;
  undefined4 *puVar4;
  undefined4 uVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_4;
  puVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649950;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_3 = 0;
  FUN_004fcdd0(param_4);
  bVar2 = FUN_0053e2f0((param_1 - param_2) * DAT_006bb45c);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    *puVar1 = 1;
    param_3 = (undefined4 *)0x2c000001;
    local_4 = 0;
    FUN_004f26d0(this,&param_3);
    local_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = local_c;
    return true;
  }
  iVar3 = FUN_0053e290(100);
  this_00 = (void *)FUN_0055bed0(2,iVar3);
  if (this_00 != (void *)0x0) {
    puVar4 = FUN_00559950(this_00,&param_3);
    local_4 = 1;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar5 = FUN_004f26f0(this);
    *puVar1 = uVar5;
  }
  ExceptionList = local_c;
  return this_00 != (void *)0x0;
}

