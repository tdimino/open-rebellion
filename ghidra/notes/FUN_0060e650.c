
void __thiscall
FUN_0060e650(void *this,uint *param_1,int param_2,int param_3,uint param_4,int param_5,byte *param_6
            ,int param_7,int param_8)

{
  uint uVar1;
  undefined3 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656ca8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_005fc100(this);
  if (uVar1 == 8) {
    param_4 = *(uint *)(*(int *)this + 0x28 + (param_4 & 0xff) * 4);
  }
  else {
    param_4 = 0xff;
  }
  param_4 = param_4 & 0xffffff;
  puVar2 = (undefined3 *)FUN_0060e600(&param_4,param_4);
  local_4 = 0;
  FUN_0060e710(this,param_1,param_2,param_3,CONCAT13((char)((uint)param_6 >> 0x18),*puVar2),param_5,
               param_6,param_7,param_8);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

