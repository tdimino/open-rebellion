
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined * __cdecl FUN_0054f5b0(int param_1)

{
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006487f8;
  local_c = ExceptionList;
  _DAT_006aafec = 0;
  local_14 = 0x14;
  local_10 = 0x18;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(&DAT_006bb2f0,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x18;
  local_10 = 0x1c;
  local_4 = 1;
  FUN_00520580(&DAT_006bb2f8,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x1c;
  local_10 = 0x20;
  local_4 = 2;
  FUN_00520580(&DAT_006bb300,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x20;
  local_10 = 0x30;
  local_4 = 3;
  FUN_00520580(&DAT_006bb308,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x2c;
  local_10 = 0x2d;
  local_4 = 4;
  FUN_00520580(&DAT_006bb310,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x30;
  local_10 = 0x40;
  local_4 = 5;
  FUN_00520580(&DAT_006bb318,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_10 = 0x14;
  local_14 = 0x10;
  local_4 = 6;
  FUN_00520580(&DAT_006bb320,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 8;
  local_10 = 0x10;
  local_4 = 7;
  FUN_00520580(&DAT_006bb328,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x40;
  local_10 = 0x80;
  local_4 = 8;
  FUN_00520580(&DAT_006bb330,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return &DAT_006bb2f0 + param_1 * 8;
}

