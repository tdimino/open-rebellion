
bool __thiscall FUN_00576a20(void *this,int *param_1,void *param_2)

{
  int iVar1;
  undefined4 local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064cb68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_00593a80(this,param_1,param_2);
  FUN_005834d0(&local_14);
  local_4 = 0;
  local_10 = CONCAT22((short)(local_10 >> 0x10),CONCAT11((byte)(local_10 >> 8) & 0xf1 | 1,3)) &
             0xffff4fff | 0x4000;
  FUN_00522a60(this,&local_14);
  local_4 = 0xffffffff;
  FUN_00583500(&local_14);
  ExceptionList = local_c;
  return iVar1 != 0;
}

